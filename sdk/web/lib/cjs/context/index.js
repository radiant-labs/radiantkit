"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.useCurrentController = exports.RadiantProvider = void 0;
const tslib_1 = require("tslib");
const react_1 = tslib_1.__importStar(require("react"));
const radiant_wasm_1 = tslib_1.__importDefault(require("radiant-wasm"));
const controller_1 = require("../controller");
const RadiantContext = (0, react_1.createContext)({
    controller: null,
    response: {},
});
function RadiantProvider({ children }) {
    const [controller, setController] = (0, react_1.useState)(null);
    const [response, setResponse] = (0, react_1.useState)({});
    const initWasm = () => tslib_1.__awaiter(this, void 0, void 0, function* () {
        console.log("Initializing wasm");
        try {
            yield (0, radiant_wasm_1.default)();
            let controller = yield controller_1.RadiantController.createController((message) => {
                console.log(message);
                setResponse(message);
            });
            setController(controller);
        }
        catch (error) {
            console.log(error);
        }
    });
    const [, setTimesRun] = (0, react_1.useState)(0);
    const counter = (0, react_1.useRef)(0);
    const effectCalled = (0, react_1.useRef)(false);
    (0, react_1.useEffect)(() => {
        if (effectCalled.current)
            return;
        counter.current += 1;
        setTimesRun(counter.current);
        effectCalled.current = true;
        initWasm();
    }, []);
    return (react_1.default.createElement(RadiantContext.Provider, { value: {
            controller,
            response,
        } }, children));
}
exports.RadiantProvider = RadiantProvider;
const useCurrentController = () => {
    return (0, react_1.useContext)(RadiantContext);
};
exports.useCurrentController = useCurrentController;
//# sourceMappingURL=index.js.map