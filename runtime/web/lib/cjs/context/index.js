"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.useCurrentController = exports.RadiantKitProvider = void 0;
const tslib_1 = require("tslib");
const react_1 = tslib_1.__importStar(require("react"));
const radiantkit_1 = tslib_1.__importDefault(require("@radiantkit/radiantkit"));
const controller_1 = require("../controller");
const RadiantKitContext = (0, react_1.createContext)({
    controller: null,
    response: {},
});
function RadiantKitProvider({ client_id, collaborate, width, height, children }) {
    const [controller, setController] = (0, react_1.useState)(null);
    const [response, setResponse] = (0, react_1.useState)({});
    const initWasm = () => tslib_1.__awaiter(this, void 0, void 0, function* () {
        try {
            yield (0, radiantkit_1.default)();
            let controller = yield controller_1.RadiantKitController.createController(client_id || BigInt(2), collaborate || false, (message) => {
                setResponse(message);
            }, width, height);
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
    return (react_1.default.createElement(RadiantKitContext.Provider, { value: {
            controller,
            response,
        } }, children));
}
exports.RadiantKitProvider = RadiantKitProvider;
const useCurrentController = () => {
    return (0, react_1.useContext)(RadiantKitContext);
};
exports.useCurrentController = useCurrentController;
//# sourceMappingURL=index.js.map