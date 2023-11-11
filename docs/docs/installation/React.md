---
sidebar_position: 1
---

# Introduction

The following guide describes how to integrate RadiantKit with your [React](https://reactjs.org/) project. We’re using [Create React App](https://reactjs.org/docs/getting-started.html) here, but the workflow should be similar with other setups.

## 1. Create a project (optional)

Let’s start with a fresh React project called **my-radiantkit-project**. [Create React App](https://reactjs.org/docs/getting-started.html) will set up everything we need.

```bash
# create a project with npm
npx create-react-app my-radiantkit-project

# change directory
cd my-radiantkit-project
```

## 2. Install the dependencies

Next, let's install **@radiantkit/react** package

```bash
npm install @radiantkit/react
```

You should now be able to start your project with npm run start, and open http://localhost:3000 in your browser.

## 3. Create a new component

To actually start using RadiantKit, we need to create a new component. Let’s call it **RadiantKit** and put the following example code in src/RadiantKit.jsx.

```js
import { RadiantKitProvider, RadiantKitCanvas } from "@radiantkit/react";

const RadiantKit = () => {
  return (
    <RadiantKitProvider>
      <RadiantKitCanvas />
    </RadiantKitProvider>
  )
}

export default RadiantKit
```

## 4. Add RadiantKit to your app

Finally, replace the content of **src/App.js** with our new **RadiantKit** component.

```js
import RadiantKit from "./RadiantKit";

function App() {
  return (
    <div className="App">
      <RadiantKit />
    </div>
  );
}

export default App;
```

## 5. Consume the `RadiantKitController` in child components

If you use the **RadiantKitProvider** to setup your RadiantKit component, you can now easily access your controller instance from any child component using the useCurrentController hook.

```js
import { useCurrentController } from '@radiantkit/react'

const RadiantKitComponent = () => {
  const { controller } = useCurrentController()

  const addRectangle = () => {
    controller && controller.addRectangle([100.0, 100.0], [100.0, 100.0])
  }

  return (
    <div>
      <button onClick={addRectangle}>Add Rectangle</button>
    </div>
  )
}

export default RadiantKitComponent
```

If you add this to `RadiantKit.jsx`, you should now see a button that adds a Rectangle to the canvas on each click.

```js
import { RadiantKitProvider, RadiantKitCanvas } from "@radiantkit/react";
import RadiantKitComponent from "./RadiantKitComponent";

const RadiantKit = () => {
  return (
    <RadiantKitProvider>
      <RadiantKitComponent />
      <RadiantKitCanvas />
    </RadiantKitProvider>
  )
}

export default RadiantKit
```