[radiant-sdk](../README.md) / [Modules](../modules.md) / [controller](../modules/controller.md) / RadiantController

# Class: RadiantController

[controller](../modules/controller.md).RadiantController

## Table of contents

### Constructors

- [constructor](controller.RadiantController.md#constructor)

### Properties

- [\_controller](controller.RadiantController.md#_controller)

### Methods

- [activateTool](controller.RadiantController.md#activatetool)
- [setFillColor](controller.RadiantController.md#setfillcolor)
- [setStrokeColor](controller.RadiantController.md#setstrokecolor)
- [setTransform](controller.RadiantController.md#settransform)
- [createController](controller.RadiantController.md#createcontroller)

## Constructors

### constructor

• **new RadiantController**(`controller`)

#### Parameters

| Name | Type |
| :------ | :------ |
| `controller` | `RadiantAppController` |

#### Defined in

[controller/index.ts:6](https://github.com/radiant-labs/radiant/blob/11e8e99/sdk/web/src/controller/index.ts#L6)

## Properties

### \_controller

• **\_controller**: `RadiantAppController`

#### Defined in

[controller/index.ts:4](https://github.com/radiant-labs/radiant/blob/11e8e99/sdk/web/src/controller/index.ts#L4)

## Methods

### activateTool

▸ **activateTool**(`tool`): `void`

Activates the provided tool.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `tool` | `string` | the tool to activate. |

#### Returns

`void`

#### Defined in

[controller/index.ts:19](https://github.com/radiant-labs/radiant/blob/11e8e99/sdk/web/src/controller/index.ts#L19)

___

### setFillColor

▸ **setFillColor**(`nodeId`, `color`): `void`

#### Parameters

| Name | Type |
| :------ | :------ |
| `nodeId` | `number` |
| `color` | `number`[] |

#### Returns

`void`

#### Defined in

[controller/index.ts:35](https://github.com/radiant-labs/radiant/blob/11e8e99/sdk/web/src/controller/index.ts#L35)

___

### setStrokeColor

▸ **setStrokeColor**(`nodeId`, `color`): `void`

#### Parameters

| Name | Type |
| :------ | :------ |
| `nodeId` | `number` |
| `color` | `number`[] |

#### Returns

`void`

#### Defined in

[controller/index.ts:44](https://github.com/radiant-labs/radiant/blob/11e8e99/sdk/web/src/controller/index.ts#L44)

___

### setTransform

▸ **setTransform**(`nodeId`, `position`, `scale`): `void`

#### Parameters

| Name | Type |
| :------ | :------ |
| `nodeId` | `number` |
| `position` | `number`[] |
| `scale` | `number`[] |

#### Returns

`void`

#### Defined in

[controller/index.ts:25](https://github.com/radiant-labs/radiant/blob/11e8e99/sdk/web/src/controller/index.ts#L25)

___

### createController

▸ `Static` **createController**(`f`): `Promise`<[`RadiantController`](controller.RadiantController.md)\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `f` | `Function` |

#### Returns

`Promise`<[`RadiantController`](controller.RadiantController.md)\>

#### Defined in

[controller/index.ts:10](https://github.com/radiant-labs/radiant/blob/11e8e99/sdk/web/src/controller/index.ts#L10)
