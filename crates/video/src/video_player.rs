// Taken from https://github.com/n00kii/egui-video

extern crate ffmpeg_the_third as ffmpeg;
use anyhow::Result;
use atomic::Atomic;
use chrono::{DateTime, Duration, Utc};
use epaint::textures::TextureOptions;
use epaint::{Color32, ColorImage, TextureHandle, Vec2};
use ffmpeg::error::EAGAIN;
use ffmpeg::ffi::AV_TIME_BASE;
use ffmpeg::format::context::input::Input;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use ffmpeg::{rescale, Rational, Rescale};
use parking_lot::Mutex;
// use ffmpeg::ffi::AVERROR;
// use ffmpeg::Packet
#[cfg(feature = "av")]
use ffmpeg::frame::Audio;
#[cfg(feature = "av")]
use ffmpeg::{software, ChannelLayout};
#[cfg(feature = "av")]
use ringbuf::SharedRb;
#[cfg(feature = "av")]
use sdl2::audio::{self, AudioCallback, AudioFormat, AudioSpecDesired};
#[cfg(feature = "av")]
use std::collections::VecDeque;
use std::sync::{Arc, Weak};
use std::time::UNIX_EPOCH;
use timer::{Guard, Timer};

// mod subtitle;
// use subtitle::Subtitle;

#[cfg(feature = "from_bytes")]
use tempfile::NamedTempFile;

#[cfg(feature = "from_bytes")]
use std::io::Write;

fn format_duration(dur: Duration) -> String {
    let dt = DateTime::<Utc>::from(UNIX_EPOCH) + dur;
    if dt.format("%H").to_string().parse::<i64>().unwrap() > 0 {
        dt.format("%H:%M:%S").to_string()
    } else {
        dt.format("%M:%S").to_string()
    }
}

/// The playback device. Needs to be initialized (and kept alive!) for use by a [`Player`].
#[cfg(feature = "av")]
pub struct AudioDevice(pub(crate) audio::AudioDevice<AudioDeviceCallback>);

#[cfg(feature = "av")]
impl AudioDevice {
    /// Create a new [`AudioDevice`] from an existing [`sdl2::AudioSubsystem`]. An [`AudioDevice`] is required for using audio.
    pub fn from_subsystem(audio_sys: &sdl2::AudioSubsystem) -> Result<AudioDevice, String> {
        let audio_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(2),
            samples: None,
        };
        let device = audio_sys.open_playback(None, &audio_spec, |_spec| AudioDeviceCallback {
            sample_streams: vec![],
        })?;
        Ok(AudioDevice(device))
    }

    /// Create a new [`AudioDevice`]. Creates an [`sdl2::AudioSubsystem`]. An [`AudioDevice`] is required for using audio.
    pub fn new() -> Result<AudioDevice, String> {
        Self::from_subsystem(&sdl2::init()?.audio()?)
    }
}

#[cfg(feature = "av")]
unsafe impl Send for AudioDevice {}
#[cfg(feature = "av")]
unsafe impl Sync for AudioDevice {}

enum PlayerMessage {
    #[allow(unused)]
    StreamCycled(Type),
}

type PlayerMessageSender = std::sync::mpsc::Sender<PlayerMessage>;
type PlayerMessageReciever = std::sync::mpsc::Receiver<PlayerMessage>;

type ApplyVideoFrameFn = Box<dyn FnMut(ColorImage) + Send>;
// type SubtitleQueue = Arc<Mutex<VecDeque<Subtitle>>>;
#[cfg(feature = "av")]
type RingbufProducer<T> = ringbuf::Producer<T, Arc<SharedRb<T, Vec<std::mem::MaybeUninit<T>>>>>;
#[cfg(feature = "av")]
type RingbufConsumer<T> = ringbuf::Consumer<T, Arc<SharedRb<T, Vec<std::mem::MaybeUninit<T>>>>>;

#[cfg(feature = "av")]
type AudioSampleProducer = RingbufProducer<f32>;
#[cfg(feature = "av")]
type AudioSampleConsumer = RingbufConsumer<f32>;

/// Configurable aspects of a [`Player`].
#[derive(Clone, Debug)]
pub struct PlayerOptions {
    /// Should the stream loop if it finishes?
    pub looping: bool,
    /// The volume of the audio stream.
    pub audio_volume: Shared<f32>,
    /// The maximum volume of the audio stream.
    pub max_audio_volume: f32,
    /// The texture options for the displayed video frame.
    pub texture_options: TextureOptions,
}

impl Default for PlayerOptions {
    fn default() -> Self {
        Self {
            looping: true,
            max_audio_volume: 1.,
            audio_volume: Shared::new(0.5),
            texture_options: TextureOptions::default(),
        }
    }
}

/// The [`Player`] processes and controls streams of video/audio. This is what you use to show a video file.
/// Initialize once, and use the [`Player::ui`] or [`Player::ui_at()`] functions to show the playback.
pub struct Player {
    /// The video streamer of the player.
    pub video_streamer: Arc<Mutex<VideoStreamer>>,
    /// The audio streamer of the player. Won't exist unless [`Player::with_audio`] is called and there exists
    /// a valid audio stream in the file.
    #[cfg(feature = "av")]
    pub audio_streamer: Option<Arc<Mutex<AudioStreamer>>>,
    /// The subtitle streamer of the player. Won't exist unless [`Player::with_subtitles`] is called and there exists
    /// a valid subtitle stream in the file.
    // pub subtitle_streamer: Option<Arc<Mutex<SubtitleStreamer>>>,
    /// The state of the player.
    pub player_state: Shared<PlayerState>,
    /// The player's texture handle.
    pub texture_handle: TextureHandle,
    /// The size of the video stream.
    pub size: Vec2,
    /// The total duration of the stream, in milliseconds.
    pub duration_ms: i64,
    /// The framerate of the video stream, in frames per second.
    pub framerate: f64,
    /// Configures certain aspects of this [`Player`].
    pub options: PlayerOptions,
    audio_stream_info: (usize, usize),
    // subtitle_stream_info: (usize, usize),
    #[allow(unused)]
    message_sender: PlayerMessageSender,
    message_reciever: PlayerMessageReciever,
    video_timer: Timer,
    #[cfg(feature = "av")]
    audio_timer: Timer,
    // subtitle_timer: Timer,
    audio_thread: Option<Guard>,
    video_thread: Option<Guard>,
    // subtitle_thread: Option<Guard>,
    last_seek_ms: Option<i64>,
    preseek_player_state: Option<PlayerState>,
    #[cfg(feature = "from_bytes")]
    temp_file: Option<NamedTempFile>,
    video_elapsed_ms: Shared<i64>,
    audio_elapsed_ms: Shared<i64>,
    // subtitle_elapsed_ms: Shared<i64>,
    video_elapsed_ms_override: Option<i64>,
    // subtitles_queue: SubtitleQueue,
    // current_subtitles: Vec<Subtitle>,
    #[allow(unused)]
    input_path: String,
}

unsafe impl Send for Player {}
unsafe impl Sync for Player {}

#[derive(PartialEq, Clone, Copy, Debug)]
/// The possible states of a [`Player`].
pub enum PlayerState {
    /// No playback.
    Stopped,
    /// Streams have reached the end of the file.
    EndOfFile,
    /// Stream is seeking. Inner bool represents whether or not the seek is currently in progress.
    Seeking(bool),
    /// Playback is paused.
    Paused,
    /// Playback is ongoing.
    Playing,
    /// Playback is scheduled to restart.
    Restarting,
}

/// Streams video.
pub struct VideoStreamer {
    video_decoder: ffmpeg::decoder::Video,
    video_stream_index: usize,
    player_state: Shared<PlayerState>,
    duration_ms: i64,
    input_context: Input,
    video_elapsed_ms: Shared<i64>,
    _audio_elapsed_ms: Shared<i64>,
    apply_video_frame_fn: Option<ApplyVideoFrameFn>,
}

/// Streams audio.
#[cfg(feature = "av")]
pub struct AudioStreamer {
    video_elapsed_ms: Shared<i64>,
    audio_elapsed_ms: Shared<i64>,
    duration_ms: i64,
    audio_decoder: ffmpeg::decoder::Audio,
    resampler: software::resampling::Context,
    audio_sample_producer: AudioSampleProducer,
    input_context: Input,
    player_state: Shared<PlayerState>,
    audio_stream_indices: VecDeque<usize>,
}

// /// Streams subtitles.
// pub struct SubtitleStreamer {
//     video_elapsed_ms: Shared<i64>,
//     _audio_elapsed_ms: Shared<i64>,
//     subtitle_elapsed_ms: Shared<i64>,
//     duration_ms: i64,
//     subtitle_decoder: ffmpeg::decoder::Subtitle,
//     next_packet: Option<Packet>,
//     subtitles_queue: SubtitleQueue,
//     input_context: Input,
//     player_state: Shared<PlayerState>,
//     subtitle_stream_indices: VecDeque<usize>,
// }

#[derive(Clone, Debug)]
/// Simple concurrecy of primitive values.
pub struct Shared<T: Copy> {
    raw_value: Arc<Atomic<T>>,
}

impl<T: Copy> Shared<T> {
    /// Set the value.
    pub fn set(&self, value: T) {
        self.raw_value.store(value, atomic::Ordering::Relaxed)
    }
    /// Get the value.
    pub fn get(&self) -> T {
        self.raw_value.load(atomic::Ordering::Relaxed)
    }
    /// Make a new cache.
    pub fn new(value: T) -> Self {
        Self {
            raw_value: Arc::new(Atomic::new(value)),
        }
    }
}

const AV_TIME_BASE_RATIONAL: Rational = Rational(1, AV_TIME_BASE);
const MILLISEC_TIME_BASE: Rational = Rational(1, 1000);

fn timestamp_to_millisec(timestamp: i64, time_base: Rational) -> i64 {
    timestamp.rescale(time_base, MILLISEC_TIME_BASE)
}

fn millisec_to_timestamp(millisec: i64, time_base: Rational) -> i64 {
    millisec.rescale(MILLISEC_TIME_BASE, time_base)
}

#[inline(always)]
fn millisec_approx_eq(a: i64, b: i64) -> bool {
    a.abs_diff(b) < 50
}

impl Player {
    /// A formatted string for displaying the duration of the video stream.
    pub fn duration_text(&mut self) -> String {
        format!(
            "{} / {}",
            format_duration(Duration::milliseconds(self.elapsed_ms())),
            format_duration(Duration::milliseconds(self.duration_ms))
        )
    }
    fn reset(&mut self) {
        self.last_seek_ms = None;
        self.video_elapsed_ms_override = None;
        self.video_elapsed_ms.set(0);
        self.audio_elapsed_ms.set(0);
        self.video_streamer.lock().reset();

        #[cfg(feature = "av")]
        if let Some(audio_decoder) = self.audio_streamer.as_mut() {
            audio_decoder.lock().reset();
        }
    }
    /// The elapsed duration of the stream, in milliseconds. This value will won't be truly accurate to the decoders
    /// while seeking, and will instead be overridden with the target seek location (for visual representation purposes).
    pub fn elapsed_ms(&self) -> i64 {
        self.video_elapsed_ms_override
            .as_ref()
            .map(|i| *i)
            .unwrap_or(self.video_elapsed_ms.get())
    }
    fn set_state(&mut self, new_state: PlayerState) {
        self.player_state.set(new_state)
    }
    /// Pause the stream.
    pub fn pause(&mut self) {
        self.set_state(PlayerState::Paused)
    }
    /// Resume the stream from a paused state.
    pub fn resume(&mut self) {
        self.set_state(PlayerState::Playing)
    }
    /// Stop the stream.
    pub fn stop(&mut self) {
        self.set_state(PlayerState::Stopped)
    }
    /// Directly stop the stream. Use if you need to immmediately end the streams, and/or you
    /// aren't able to call the player's [`Player::ui`]/[`Player::ui_at`] functions later on.
    pub fn stop_direct(&mut self) {
        self.video_thread = None;
        self.audio_thread = None;
        self.reset()
    }
    // fn duration_frac(&mut self) -> f32 {
    //     self.elapsed_ms() as f32 / self.duration_ms as f32
    // }
    /// Seek to a location in the stream.
    pub fn seek(&mut self, seek_frac: f32) {
        let current_state = self.player_state.get();
        if !matches!(current_state, PlayerState::Seeking(true)) {
            match current_state {
                PlayerState::Stopped | PlayerState::EndOfFile => {
                    self.preseek_player_state = Some(PlayerState::Paused);
                    self.start();
                }
                PlayerState::Paused | PlayerState::Playing => {
                    self.preseek_player_state = Some(current_state);
                }
                _ => (),
            }

            let video_streamer = self.video_streamer.clone();
            // let mut subtitle_streamer = self.subtitle_streamer.clone();
            // let subtitle_queue = self.subtitles_queue.clone();

            self.last_seek_ms = Some((seek_frac as f64 * self.duration_ms as f64) as i64);
            self.set_state(PlayerState::Seeking(true));

            #[cfg(feature = "av")]
            {
                let mut audio_streamer = self.audio_streamer.clone();
                if let Some(audio_streamer) = audio_streamer.take() {
                    std::thread::spawn(move || {
                        audio_streamer.lock().seek(seek_frac);
                    });
                };
            }
            // if let Some(subtitle_streamer) = subtitle_streamer.take() {
            //     self.current_subtitles.clear();
            //     std::thread::spawn(move || {
            //         subtitle_queue.lock().clear();
            //         subtitle_streamer.lock().seek(seek_frac);
            //     });
            // };
            std::thread::spawn(move || {
                video_streamer.lock().seek(seek_frac);
            });
        }
    }
    fn spawn_timers(&mut self) {
        let mut texture_handle = self.texture_handle.clone();
        let texture_options = self.options.texture_options.clone();
        let wait_duration = Duration::milliseconds((1000. / self.framerate) as i64);

        fn play<T: Streamer>(streamer: &Weak<Mutex<T>>) {
            if let Some(streamer) = streamer.upgrade() {
                if let Some(mut streamer) = streamer.try_lock() {
                    if (streamer.player_state().get() == PlayerState::Playing)
                        && streamer.primary_elapsed_ms().get() >= streamer.elapsed_ms().get()
                    {
                        match streamer.recieve_next_packet_until_frame() {
                            Ok(frame) => streamer.apply_frame(frame),
                            Err(e) => {
                                if is_ffmpeg_eof_error(&e) && streamer.is_primary_streamer() {
                                    streamer.player_state().set(PlayerState::EndOfFile)
                                }
                            }
                        }
                    }
                }
            }
        }

        self.video_streamer.lock().apply_video_frame_fn = Some(Box::new(move |frame| {
            texture_handle.set(frame, texture_options)
        }));

        let video_streamer_ref = Arc::downgrade(&self.video_streamer);

        let video_timer_guard = self.video_timer.schedule_repeating(wait_duration, move || {
            play(&video_streamer_ref);
            // ctx.request_repaint();
        });

        self.video_thread = Some(video_timer_guard);

        #[cfg(feature = "av")]
        if let Some(audio_decoder) = self.audio_streamer.as_ref() {
            let audio_decoder_ref = Arc::downgrade(&audio_decoder);
            let audio_timer_guard = self
                .audio_timer
                .schedule_repeating(Duration::zero(), move || play(&audio_decoder_ref));
            self.audio_thread = Some(audio_timer_guard);
        }

        // if let Some(subtitle_decoder) = self.subtitle_streamer.as_ref() {
        //     let subtitle_decoder_ref = Arc::downgrade(&subtitle_decoder);
        //     let subtitle_timer_guard = self
        //         .subtitle_timer
        //         .schedule_repeating(wait_duration, move || play(&subtitle_decoder_ref));
        //     self.subtitle_thread = Some(subtitle_timer_guard);
        // }
    }
    /// Start the stream.
    pub fn start(&mut self) {
        self.stop_direct();
        self.spawn_timers();
        self.resume();
    }

    /// Process player state updates. This function must be called for proper function
    /// of the player. This function is already included in  [`Player::ui`] or
    /// [`Player::ui_at`].
    pub fn process_state(&mut self) {
        let mut reset_stream = false;

        match self.player_state.get() {
            PlayerState::EndOfFile => {
                if self.options.looping {
                    reset_stream = true;
                } else {
                    self.player_state.set(PlayerState::Stopped);
                }
            }
            PlayerState::Stopped => {
                self.stop_direct();
            }
            PlayerState::Playing => {
                // for subtitle in self.current_subtitles.iter_mut() {
                //     subtitle.remaining_duration_ms -=
                //         self.ctx_ref.input(|i| (i.stable_dt * 1000.) as i64);
                // }
                // self.current_subtitles
                //     .retain(|s| s.remaining_duration_ms > 0);
                // if let Some(mut queue) = self.subtitles_queue.try_lock() {
                //     if queue.len() > 1 {
                //         self.current_subtitles.push(queue.pop_front().unwrap());
                //     }
                // }
            }
            PlayerState::Seeking(seek_in_progress) => {
                if self.last_seek_ms.is_some() {
                    let last_seek_ms = *self.last_seek_ms.as_ref().unwrap();
                    if !seek_in_progress {
                        if let Some(previeous_player_state) = self.preseek_player_state {
                            self.set_state(previeous_player_state)
                        }
                        self.video_elapsed_ms_override = None;
                        self.last_seek_ms = None;
                    } else {
                        self.video_elapsed_ms_override = Some(last_seek_ms);
                    }
                } else {
                    self.video_elapsed_ms_override = None;
                }
            }
            PlayerState::Restarting => reset_stream = true,
            _ => (),
        }
        if let Ok(message) = self.message_reciever.try_recv() {
            fn increment_stream_info(stream_info: &mut (usize, usize)) {
                stream_info.0 = ((stream_info.0 + 1) % (stream_info.1 + 1)).max(1);
            }
            match message {
                PlayerMessage::StreamCycled(stream_type) => match stream_type {
                    Type::Audio => increment_stream_info(&mut self.audio_stream_info),
                    Type::Subtitle => {
                        // self.current_subtitles.clear();
                        // increment_stream_info(&mut self.subtitle_stream_info);
                    }
                    _ => unreachable!(),
                },
            }
        }
        if reset_stream {
            self.reset();
            self.resume();
        }
    }

    #[cfg(feature = "from_bytes")]
    /// Create a new [`Player`] from input bytes.
    pub fn from_bytes(ctx: &egui::Context, input_bytes: &[u8]) -> Result<Self> {
        let mut file = tempfile::Builder::new().tempfile()?;
        file.write_all(input_bytes)?;
        let path = file.path().to_string_lossy().to_string();
        let mut slf = Self::new(ctx, &path)?;
        slf.temp_file = Some(file);
        Ok(slf)
    }

    /// Initializes the audio stream (if there is one), required for making a [`Player`] output audio.
    /// Will stop and reset the player's state.
    #[cfg(feature = "av")]
    pub fn add_audio(&mut self, audio_device: &mut AudioDevice) -> Result<()> {
        let audio_input_context = input(&self.input_path)?;
        let audio_stream_indices = get_stream_indices_of_type(&audio_input_context, Type::Audio);

        let audio_streamer = if !audio_stream_indices.is_empty() {
            let audio_decoder =
                get_decoder_from_stream_index(&audio_input_context, audio_stream_indices[0])?
                    .audio()?;

            let audio_sample_buffer =
                SharedRb::<f32, Vec<_>>::new(audio_device.0.spec().size as usize);
            let (audio_sample_producer, audio_sample_consumer) = audio_sample_buffer.split();
            let audio_resampler = ffmpeg::software::resampling::context::Context::get(
                audio_decoder.format(),
                audio_decoder.channel_layout(),
                audio_decoder.rate(),
                audio_device.0.spec().format.to_sample(),
                ChannelLayout::STEREO,
                audio_device.0.spec().freq as u32,
            )?;

            audio_device
                .0
                .lock()
                .sample_streams
                .push(AudioSampleStream {
                    sample_consumer: audio_sample_consumer,
                    audio_volume: self.options.audio_volume.clone(),
                });

            audio_device.0.resume();

            self.stop_direct();
            self.audio_stream_info = (1, audio_stream_indices.len()); // first stream, out of all the other streams
            Some(AudioStreamer {
                duration_ms: self.duration_ms,
                player_state: self.player_state.clone(),
                video_elapsed_ms: self.video_elapsed_ms.clone(),
                audio_elapsed_ms: self.audio_elapsed_ms.clone(),
                audio_sample_producer,
                input_context: audio_input_context,
                audio_decoder,
                resampler: audio_resampler,
                audio_stream_indices,
            })
        } else {
            None
        };
        self.audio_streamer = audio_streamer.map(|s| Arc::new(Mutex::new(s)));
        Ok(())
    }

    /// Initializes the subtitle stream (if there is one), required for making a [`Player`] display subtitles.
    /// Will stop and reset the player's state.
    // pub fn add_subtitles(&mut self) -> Result<()> {
    //     let subtitle_input_context = input(&self.input_path)?;
    //     let subtitle_stream_indices =
    //         get_stream_indices_of_type(&subtitle_input_context, Type::Subtitle);

    //     let subtitle_streamer = if !subtitle_stream_indices.is_empty() {
    //         let subtitle_decoder =
    //             get_decoder_from_stream_index(&subtitle_input_context, subtitle_stream_indices[0])?
    //                 .subtitle()?;

    //         self.stop_direct();
    //         self.subtitle_stream_info = (1, subtitle_stream_indices.len()); // first stream, out of all the other streams
    //         Some(SubtitleStreamer {
    //             next_packet: None,
    //             duration_ms: self.duration_ms,
    //             player_state: self.player_state.clone(),
    //             video_elapsed_ms: self.video_elapsed_ms.clone(),
    //             _audio_elapsed_ms: self.audio_elapsed_ms.clone(),
    //             subtitle_elapsed_ms: self.subtitle_elapsed_ms.clone(),
    //             input_context: subtitle_input_context,
    //             subtitles_queue: self.subtitles_queue.clone(),
    //             subtitle_decoder,
    //             subtitle_stream_indices,
    //         })
    //     } else {
    //         None
    //     };
    //     self.subtitle_streamer = subtitle_streamer.map(|s| Arc::new(Mutex::new(s)));
    //     Ok(())
    // }

    #[cfg(feature = "av")]
    fn cycle_stream<T: Streamer + 'static>(&self, mut streamer: Option<&Arc<Mutex<T>>>) {
        if let Some(streamer) = streamer.take() {
            let message_sender = self.message_sender.clone();
            let streamer = streamer.clone();
            std::thread::spawn(move || {
                let mut streamer = streamer.lock();
                streamer.cycle_stream();
                message_sender.send(PlayerMessage::StreamCycled(streamer.stream_type()))
            });
        };
    }

    /// Switches to the next subtitle stream.
    // pub fn cycle_subtitle_stream(&mut self) {
    //     self.cycle_stream(self.subtitle_streamer.as_ref());
    // }

    /// Switches to the next audio stream.
    #[cfg(feature = "av")]
    pub fn cycle_audio_stream(&mut self) {
        self.cycle_stream(self.audio_streamer.as_ref());
    }

    /// Enables using [`Player::add_audio`] with the builder pattern.
    #[cfg(feature = "av")]
    pub fn with_audio(mut self, audio_device: &mut AudioDevice) -> Result<Self> {
        self.add_audio(audio_device)?;
        Ok(self)
    }

    /// Enables using [`Player::add_subtitles`] with the builder pattern.
    // pub fn with_subtitles(mut self) -> Result<Self> {
    //     self.add_subtitles()?;
    //     Ok(self)
    // }

    /// Create a new [`Player`].
    pub fn new(input_path: &String, texture_handle: TextureHandle) -> Result<Self> {
        let input_context = input(&input_path)?;
        let video_stream = input_context
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = video_stream.index();

        let video_elapsed_ms = Shared::new(0);
        let audio_elapsed_ms = Shared::new(0);
        let player_state = Shared::new(PlayerState::Stopped);

        let video_context =
            ffmpeg::codec::context::Context::from_parameters(video_stream.parameters())?;
        let video_decoder = video_context.decoder().video()?;
        let framerate = (video_stream.avg_frame_rate().numerator() as f64)
            / video_stream.avg_frame_rate().denominator() as f64;

        let (width, height) = (video_decoder.width(), video_decoder.height());
        let size = Vec2::new(width as f32, height as f32);
        let duration_ms = timestamp_to_millisec(input_context.duration(), AV_TIME_BASE_RATIONAL); // in sec

        let stream_decoder = VideoStreamer {
            apply_video_frame_fn: None,
            duration_ms,
            video_decoder,
            video_stream_index,
            _audio_elapsed_ms: audio_elapsed_ms.clone(),
            video_elapsed_ms: video_elapsed_ms.clone(),
            input_context,
            player_state: player_state.clone(),
        };
        let options = PlayerOptions::default();
        // let texture_handle =
        //     ctx.load_texture("vidstream", ColorImage::example(), options.texture_options);
        let (message_sender, message_reciever) = std::sync::mpsc::channel();
        let mut streamer = Self {
            input_path: input_path.clone(),
            #[cfg(feature = "av")]
            audio_streamer: None,
            // subtitle_streamer: None,
            video_streamer: Arc::new(Mutex::new(stream_decoder)),
            // subtitle_stream_info: (0, 0),
            audio_stream_info: (0, 0),
            framerate,
            video_timer: Timer::new(),
            #[cfg(feature = "av")]
            audio_timer: Timer::new(),
            // subtitle_timer: Timer::new(),
            // subtitle_elapsed_ms: Shared::new(0),
            preseek_player_state: None,
            video_thread: None,
            // subtitle_thread: None,
            audio_thread: None,
            texture_handle,
            player_state,
            message_sender,
            message_reciever,
            video_elapsed_ms,
            audio_elapsed_ms,
            size,
            last_seek_ms: None,
            duration_ms,
            options,
            video_elapsed_ms_override: None,
            // ctx_ref: ctx.clone(),
            // subtitles_queue: Arc::new(Mutex::new(VecDeque::new())),
            // current_subtitles: Vec::new(),
            #[cfg(feature = "from_bytes")]
            temp_file: None,
        };

        loop {
            if let Ok(_texture_handle) = streamer.try_set_texture_handle() {
                break;
            }
        }

        Ok(streamer)
    }

    fn try_set_texture_handle(&mut self) -> Result<TextureHandle> {
        match self.video_streamer.lock().recieve_next_packet_until_frame() {
            Ok(first_frame) => {
                // let texture_handle = self.ctx_ref.load_texture(
                //     "vidstream",
                //     first_frame,
                //     self.options.texture_options,
                // );
                self.texture_handle
                    .set(first_frame, self.options.texture_options);
                Ok(self.texture_handle.clone())
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(feature = "av")]
fn get_stream_indices_of_type(
    input_context: &Input,
    stream_type: ffmpeg::media::Type,
) -> VecDeque<usize> {
    input_context
        .streams()
        .filter_map(|s| (s.parameters().medium() == stream_type).then_some(s.index()))
        .collect::<VecDeque<_>>()
}

#[cfg(feature = "av")]
fn get_decoder_from_stream_index(
    input_context: &Input,
    stream_index: usize,
) -> Result<ffmpeg::decoder::Decoder> {
    let context = ffmpeg::codec::context::Context::from_parameters(
        input_context.stream(stream_index).unwrap().parameters(),
    )?;
    Ok(context.decoder())
}

/// Streams data.
pub trait Streamer: Send {
    /// The associated type of frame used for the stream.
    type Frame;
    /// The associated type after the frame is processed.
    type ProcessedFrame;
    /// Seek to a location within the stream.
    fn seek(&mut self, seek_frac: f32) {
        let target_ms = (seek_frac as f64 * self.duration_ms() as f64) as i64;
        let seek_completed = millisec_approx_eq(target_ms, self.elapsed_ms().get());
        // stop seeking near target so we dont waste cpu cycles
        if !seek_completed {
            let elapsed_ms = self.elapsed_ms().clone();
            let currently_behind_target = || elapsed_ms.get() < target_ms;

            let seeking_backwards = target_ms < self.elapsed_ms().get();
            let target_ts = millisec_to_timestamp(target_ms, rescale::TIME_BASE);

            if let Err(_) = self.input_context().seek(target_ts, ..target_ts) {
                // dbg!(e); TODO: propogate error
            } else {
                self.decoder().flush();
                let mut previous_elapsed_ms = self.elapsed_ms().get();

                // this drop frame loop lets us refresh until current_ts is accurate
                if seeking_backwards {
                    while !currently_behind_target() {
                        let next_elapsed_ms = self.elapsed_ms().get();
                        if next_elapsed_ms > previous_elapsed_ms {
                            break;
                        }
                        previous_elapsed_ms = next_elapsed_ms;
                        if let Err(e) = self.drop_frames() {
                            if is_ffmpeg_eof_error(&e) {
                                break;
                            }
                        }
                    }
                }

                // // this drop frame loop drops frames until we are at desired
                while currently_behind_target() {
                    if let Err(e) = self.drop_frames() {
                        if is_ffmpeg_eof_error(&e) {
                            break;
                        }
                    }
                }

                // frame preview
                if self.is_primary_streamer() {
                    match self.recieve_next_packet_until_frame() {
                        Ok(frame) => self.apply_frame(frame),
                        _ => (),
                    }
                }
            }
        }
        if self.is_primary_streamer() {
            self.player_state().set(PlayerState::Seeking(false));
        }
    }
    /// The type of data this stream corresponds to.
    fn stream_type(&self) -> Type;
    /// The primary streamer will control most of the state/syncing.
    fn is_primary_streamer(&self) -> bool;
    /// The stream index.
    fn stream_index(&self) -> usize;
    /// Move to the next stream index, if possible, and return the new_stream_index.
    fn cycle_stream(&mut self) -> usize;
    /// The elapsed time of this streamer, in milliseconds.
    fn elapsed_ms(&self) -> &Shared<i64>;
    /// The elapsed time of the primary streamer, in milliseconds.
    fn primary_elapsed_ms(&self) -> &Shared<i64>;
    /// The total duration of the stream, in milliseconds.
    fn duration_ms(&self) -> i64;
    /// The streamer's decoder.
    fn decoder(&mut self) -> &mut ffmpeg::decoder::Opened;
    /// The streamer's input context.
    fn input_context(&mut self) -> &mut ffmpeg::format::context::Input;
    /// The streamer's state.
    fn player_state(&self) -> &Shared<PlayerState>;
    /// Output a frame from the decoder.
    fn decode_frame(&mut self) -> Result<Self::Frame>;
    /// Ignore the remainder of this packet.
    fn drop_frames(&mut self) -> Result<()> {
        if self.decode_frame().is_err() {
            self.recieve_next_packet()
        } else {
            self.drop_frames()
        }
    }
    /// Recieve the next packet of the stream.
    fn recieve_next_packet(&mut self) -> Result<()> {
        if let Some((stream, packet)) = self.input_context().packets().next() {
            let time_base = stream.time_base();
            if stream.index() == self.stream_index() {
                self.decoder().send_packet(&packet)?;
                if let Some(dts) = packet.dts() {
                    self.elapsed_ms().set(timestamp_to_millisec(dts, time_base));
                }
            }
        } else {
            self.decoder().send_eof()?;
        }
        Ok(())
    }
    /// Reset the stream to its initial state.
    fn reset(&mut self) {
        let beginning: i64 = 0;
        let beginning_seek = beginning.rescale((1, 1), rescale::TIME_BASE);
        let _ = self.input_context().seek(beginning_seek, ..beginning_seek);
        self.decoder().flush();
    }
    /// Keep recieving packets until a frame can be decoded.
    fn recieve_next_packet_until_frame(&mut self) -> Result<Self::ProcessedFrame> {
        match self.recieve_next_frame() {
            Ok(frame_result) => Ok(frame_result),
            Err(e) => {
                // dbg!(&e, is_ffmpeg_incomplete_error(&e));
                if is_ffmpeg_incomplete_error(&e) {
                    self.recieve_next_packet()?;
                    self.recieve_next_packet_until_frame()
                } else {
                    Err(e)
                }
            }
        }
    }
    /// Process a decoded frame.
    fn process_frame(&mut self, frame: Self::Frame) -> Result<Self::ProcessedFrame>;
    /// Apply a processed frame
    fn apply_frame(&mut self, _frame: Self::ProcessedFrame) {}
    /// Decode and process a frame.
    fn recieve_next_frame(&mut self) -> Result<Self::ProcessedFrame> {
        match self.decode_frame() {
            Ok(decoded_frame) => self.process_frame(decoded_frame),
            Err(e) => {
                return Err(e);
            }
        }
    }
}

impl Streamer for VideoStreamer {
    type Frame = Video;
    type ProcessedFrame = ColorImage;
    fn stream_type(&self) -> Type {
        Type::Video
    }
    fn is_primary_streamer(&self) -> bool {
        true
    }
    fn stream_index(&self) -> usize {
        self.video_stream_index
    }
    fn cycle_stream(&mut self) -> usize {
        0
    }
    fn decoder(&mut self) -> &mut ffmpeg::decoder::Opened {
        &mut self.video_decoder.0
    }
    fn input_context(&mut self) -> &mut ffmpeg::format::context::Input {
        &mut self.input_context
    }
    fn elapsed_ms(&self) -> &Shared<i64> {
        &self.video_elapsed_ms
    }
    fn primary_elapsed_ms(&self) -> &Shared<i64> {
        &self.video_elapsed_ms
    }
    fn duration_ms(&self) -> i64 {
        self.duration_ms
    }
    fn player_state(&self) -> &Shared<PlayerState> {
        &self.player_state
    }
    fn decode_frame(&mut self) -> Result<Self::Frame> {
        let mut decoded_frame = Video::empty();
        self.video_decoder.receive_frame(&mut decoded_frame)?;
        Ok(decoded_frame)
    }
    fn apply_frame(&mut self, frame: Self::ProcessedFrame) {
        if let Some(apply_video_frame_fn) = self.apply_video_frame_fn.as_mut() {
            apply_video_frame_fn(frame)
        }
    }
    fn process_frame(&mut self, frame: Self::Frame) -> Result<Self::ProcessedFrame> {
        let mut rgb_frame = Video::empty();
        let mut scaler = Context::get(
            frame.format(),
            frame.width(),
            frame.height(),
            Pixel::RGB24,
            frame.width(),
            frame.height(),
            Flags::BILINEAR,
        )?;
        scaler.run(&frame, &mut rgb_frame)?;

        let image = video_frame_to_image(rgb_frame);
        Ok(image)
    }
}

#[cfg(feature = "av")]
impl Streamer for AudioStreamer {
    type Frame = Audio;
    type ProcessedFrame = ();
    fn stream_type(&self) -> Type {
        Type::Audio
    }
    fn is_primary_streamer(&self) -> bool {
        false
    }
    fn stream_index(&self) -> usize {
        self.audio_stream_indices[0]
    }
    fn cycle_stream(&mut self) -> usize {
        self.audio_stream_indices.rotate_right(1);
        let new_stream_index = self.stream_index();
        let new_decoder = get_decoder_from_stream_index(&self.input_context, new_stream_index)
            .unwrap()
            .audio()
            .unwrap();
        let new_resampler = ffmpeg::software::resampling::context::Context::get(
            new_decoder.format(),
            new_decoder.channel_layout(),
            new_decoder.rate(),
            self.resampler.output().format,
            ChannelLayout::STEREO,
            self.resampler.output().rate,
        )
        .unwrap();
        self.audio_decoder = new_decoder;
        self.resampler = new_resampler;
        new_stream_index
    }
    fn decoder(&mut self) -> &mut ffmpeg::decoder::Opened {
        &mut self.audio_decoder.0
    }
    fn input_context(&mut self) -> &mut ffmpeg::format::context::Input {
        &mut self.input_context
    }
    fn elapsed_ms(&self) -> &Shared<i64> {
        &self.audio_elapsed_ms
    }
    fn primary_elapsed_ms(&self) -> &Shared<i64> {
        &self.video_elapsed_ms
    }
    fn duration_ms(&self) -> i64 {
        self.duration_ms
    }
    fn player_state(&self) -> &Shared<PlayerState> {
        &self.player_state
    }
    fn decode_frame(&mut self) -> Result<Self::Frame> {
        let mut decoded_frame = Audio::empty();
        self.audio_decoder.receive_frame(&mut decoded_frame)?;
        Ok(decoded_frame)
    }
    fn process_frame(&mut self, frame: Self::Frame) -> Result<Self::ProcessedFrame> {
        let mut resampled_frame = ffmpeg::frame::Audio::empty();
        self.resampler.run(&frame, &mut resampled_frame)?;
        let audio_samples = if resampled_frame.is_packed() {
            packed(&resampled_frame)
        } else {
            resampled_frame.plane(0)
        };
        while self.audio_sample_producer.free_len() < audio_samples.len() {
            // std::thread::sleep(std::time::Duration::from_millis(10));
        }
        self.audio_sample_producer.push_slice(audio_samples);
        Ok(())
    }
}

// impl Streamer for SubtitleStreamer {
//     type Frame = (ffmpeg::codec::subtitle::Subtitle, i64);
//     type ProcessedFrame = Subtitle;
//     fn stream_type(&self) -> Type {
//         Type::Subtitle
//     }
//     fn is_primary_streamer(&self) -> bool {
//         false
//     }
//     fn stream_index(&self) -> usize {
//         self.subtitle_stream_indices[0]
//     }
//     fn cycle_stream(&mut self) -> usize {
//         self.subtitle_stream_indices.rotate_right(1);
//         self.subtitle_decoder.flush();
//         let new_stream_index = self.stream_index();
//         let new_decoder = get_decoder_from_stream_index(&self.input_context, new_stream_index)
//             .unwrap()
//             .subtitle()
//             .unwrap();
//         self.next_packet = None;
//         // bandaid: subtitle decoder is always ahead of video decoder, so we need to seek it back to the
//         // video decoder's location in order so that we don't miss possible subtitles when switching streams
//         self.seek(self.primary_elapsed_ms().get() as f32 / self.duration_ms as f32);
//         self.subtitles_queue.lock().clear();
//         self.subtitle_decoder = new_decoder;
//         new_stream_index
//     }
//     fn decoder(&mut self) -> &mut ffmpeg::decoder::Opened {
//         &mut self.subtitle_decoder.0
//     }
//     fn input_context(&mut self) -> &mut ffmpeg::format::context::Input {
//         &mut self.input_context
//     }
//     fn elapsed_ms(&self) -> &Shared<i64> {
//         &self.subtitle_elapsed_ms
//     }
//     fn primary_elapsed_ms(&self) -> &Shared<i64> {
//         &self.video_elapsed_ms
//     }
//     fn duration_ms(&self) -> i64 {
//         self.duration_ms
//     }
//     fn player_state(&self) -> &Shared<PlayerState> {
//         &self.player_state
//     }
//     fn recieve_next_packet(&mut self) -> Result<()> {
//         if let Some((stream, packet)) = self.input_context().packets().next() {
//             let time_base = stream.time_base();
//             if stream.index() == self.stream_index() {
//                 if let Some(dts) = packet.dts() {
//                     self.elapsed_ms().set(timestamp_to_millisec(dts, time_base));
//                 }
//                 self.next_packet = Some(packet);
//             }
//         } else {
//             self.decoder().send_eof()?;
//         }
//         Ok(())
//     }
//     fn decode_frame(&mut self) -> Result<Self::Frame> {
//         if let Some(packet) = self.next_packet.take() {
//             let mut decoded_frame = ffmpeg::Subtitle::new();
//             self.subtitle_decoder.decode(&packet, &mut decoded_frame)?;
//             Ok((decoded_frame, packet.duration()))
//         } else {
//             Err(ffmpeg::Error::from(AVERROR(EAGAIN)).into())
//         }
//     }
//     fn process_frame(&mut self, frame: Self::Frame) -> Result<Self::ProcessedFrame> {
//         // TODO: manage the case when frame rects len > 1
//         let (frame, duration) = frame;
//         if let Some(rect) = frame.rects().next() {
//             Subtitle::from_ffmpeg_rect(rect).map(|s| {
//                 if s.remaining_duration_ms == 0 {
//                     s.with_duration_ms(duration)
//                 } else {
//                     s
//                 }
//             })
//         } else {
//             anyhow::bail!("no subtitle")
//         }
//     }
//     fn apply_frame(&mut self, frame: Self::ProcessedFrame) {
//         let mut queue = self.subtitles_queue.lock();
//         queue.push_back(frame)
//     }
// }

#[cfg(feature = "av")]
type FfmpegAudioFormat = ffmpeg::format::Sample;
#[cfg(feature = "av")]
type FfmpegAudioFormatType = ffmpeg::format::sample::Type;

#[cfg(feature = "av")]
trait AsFfmpegSample {
    fn to_sample(&self) -> FfmpegAudioFormat;
}

#[cfg(feature = "av")]
impl AsFfmpegSample for AudioFormat {
    fn to_sample(&self) -> FfmpegAudioFormat {
        match self {
            AudioFormat::U8 => FfmpegAudioFormat::U8(FfmpegAudioFormatType::Packed),
            AudioFormat::S8 => panic!("unsupported audio format"),
            AudioFormat::U16LSB => panic!("unsupported audio format"),
            AudioFormat::U16MSB => panic!("unsupported audio format"),
            AudioFormat::S16LSB => FfmpegAudioFormat::I16(FfmpegAudioFormatType::Packed),
            AudioFormat::S16MSB => FfmpegAudioFormat::I16(FfmpegAudioFormatType::Packed),
            AudioFormat::S32LSB => FfmpegAudioFormat::I32(FfmpegAudioFormatType::Packed),
            AudioFormat::S32MSB => FfmpegAudioFormat::I32(FfmpegAudioFormatType::Packed),
            AudioFormat::F32LSB => FfmpegAudioFormat::F32(FfmpegAudioFormatType::Packed),
            AudioFormat::F32MSB => FfmpegAudioFormat::F32(FfmpegAudioFormatType::Packed),
        }
    }
}

/// Pipes audio samples to SDL2.
#[cfg(feature = "av")]
pub struct AudioDeviceCallback {
    sample_streams: Vec<AudioSampleStream>,
}

#[cfg(feature = "av")]
struct AudioSampleStream {
    sample_consumer: AudioSampleConsumer,
    audio_volume: Shared<f32>,
}

#[cfg(feature = "av")]
impl AudioCallback for AudioDeviceCallback {
    type Channel = f32;
    fn callback(&mut self, output: &mut [Self::Channel]) {
        for x in output.iter_mut() {
            *x = self
                .sample_streams
                .iter_mut()
                .map(|s| s.sample_consumer.pop().unwrap_or(0.) * s.audio_volume.get())
                .sum()
        }
    }
}

#[inline]
// Thanks https://github.com/zmwangx/rust-ffmpeg/issues/72 <3
// Interpret the audio frame's data as packed (alternating channels, 12121212, as opposed to planar 11112222)
#[cfg(feature = "av")]
fn packed<T: ffmpeg::frame::audio::Sample>(frame: &ffmpeg::frame::Audio) -> &[T] {
    if !frame.is_packed() {
        panic!("data is not packed");
    }

    if !<T as ffmpeg::frame::audio::Sample>::is_valid(frame.format(), frame.channels()) {
        panic!("unsupported type");
    }

    unsafe {
        std::slice::from_raw_parts(
            (*frame.as_ptr()).data[0] as *const T,
            frame.samples() * frame.channels() as usize,
        )
    }
}

fn is_ffmpeg_eof_error(error: &anyhow::Error) -> bool {
    matches!(
        error.downcast_ref::<ffmpeg::Error>(),
        Some(ffmpeg::Error::Eof)
    )
}

fn is_ffmpeg_incomplete_error(error: &anyhow::Error) -> bool {
    matches!(
        error.downcast_ref::<ffmpeg::Error>(),
        Some(ffmpeg::Error::Other { errno } ) if *errno == EAGAIN
    )
}

fn video_frame_to_image(frame: Video) -> ColorImage {
    let size = [frame.width() as usize, frame.height() as usize];
    let data = frame.data(0);
    let stride = frame.stride(0);
    let pixel_size_bytes = 3;
    let byte_width: usize = pixel_size_bytes * frame.width() as usize;
    let height: usize = frame.height() as usize;
    let mut pixels = vec![];
    for line in 0..height {
        let begin = line * stride;
        let end = begin + byte_width;
        let data_line = &data[begin..end];
        pixels.extend(
            data_line
                .chunks_exact(pixel_size_bytes)
                .map(|p| Color32::from_rgb(p[0], p[1], p[2])),
        )
    }
    ColorImage { size, pixels }
}
