# Setlist

- [1. Property `Setlist > presets`](#presets)
  - [1.1. Setlist > presets > PresetConfig](#presets_items)
    - [1.1.1. Property `Setlist > presets > presets items > analog`](#presets_items_analog)
      - [1.1.1.1. Property `Setlist > presets > presets items > analog > AnalogYamlConfig`](#presets_items_analog_additionalProperties)
        - [1.1.1.1.1. Property `Setlist > presets > presets items > analog > additionalProperties > cc`](#presets_items_analog_additionalProperties_cc)
        - [1.1.1.1.2. Property `Setlist > presets > presets items > analog > additionalProperties > channel`](#presets_items_analog_additionalProperties_channel)
        - [1.1.1.1.3. Property `Setlist > presets > presets items > analog > additionalProperties > label`](#presets_items_analog_additionalProperties_label)
        - [1.1.1.1.4. Property `Setlist > presets > presets items > analog > additionalProperties > max`](#presets_items_analog_additionalProperties_max)
        - [1.1.1.1.5. Property `Setlist > presets > presets items > analog > additionalProperties > min`](#presets_items_analog_additionalProperties_min)
    - [1.1.2. Property `Setlist > presets > presets items > buttons`](#presets_items_buttons)
      - [1.1.2.1. Property `Setlist > presets > presets items > buttons > ButtonConfig`](#presets_items_buttons_additionalProperties)
        - [1.1.2.1.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions`](#presets_items_buttons_additionalProperties_actions)
          - [1.1.2.1.1.1. Setlist > presets > presets items > buttons > additionalProperties > actions > ActionYaml](#presets_items_buttons_additionalProperties_actions_items)
            - [1.1.2.1.1.1.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 0`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i0)
              - [1.1.2.1.1.1.1.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 0 > delay`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay)
            - [1.1.2.1.1.1.2. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 1`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i1)
              - [1.1.2.1.1.1.2.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 1 > cc`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc)
              - [1.1.2.1.1.1.2.2. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 1 > channel`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel)
              - [1.1.2.1.1.1.2.3. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 1 > value`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_value)
            - [1.1.2.1.1.1.3. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 2`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i2)
              - [1.1.2.1.1.1.3.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 2 > channel`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel)
              - [1.1.2.1.1.1.3.2. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 2 > program_change`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change)
            - [1.1.2.1.1.1.4. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 3`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i3)
              - [1.1.2.1.1.1.4.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 3 > channel`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel)
              - [1.1.2.1.1.1.4.2. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 3 > note`](#presets_items_buttons_additionalProperties_actions_items_anyOf_i3_note)
        - [1.1.2.1.2. Property `Setlist > presets > presets items > buttons > additionalProperties > animation`](#presets_items_buttons_additionalProperties_animation)
        - [1.1.2.1.3. Property `Setlist > presets > presets items > buttons > additionalProperties > cc`](#presets_items_buttons_additionalProperties_cc)
        - [1.1.2.1.4. Property `Setlist > presets > presets items > buttons > additionalProperties > channel`](#presets_items_buttons_additionalProperties_channel)
        - [1.1.2.1.5. Property `Setlist > presets > presets items > buttons > additionalProperties > color`](#presets_items_buttons_additionalProperties_color)
        - [1.1.2.1.6. Property `Setlist > presets > presets items > buttons > additionalProperties > label`](#presets_items_buttons_additionalProperties_label)
        - [1.1.2.1.7. Property `Setlist > presets > presets items > buttons > additionalProperties > level`](#presets_items_buttons_additionalProperties_level)
        - [1.1.2.1.8. Property `Setlist > presets > presets items > buttons > additionalProperties > note`](#presets_items_buttons_additionalProperties_note)
        - [1.1.2.1.9. Property `Setlist > presets > presets items > buttons > additionalProperties > on_long_press`](#presets_items_buttons_additionalProperties_on_long_press)
        - [1.1.2.1.10. Property `Setlist > presets > presets items > buttons > additionalProperties > program_change`](#presets_items_buttons_additionalProperties_program_change)
        - [1.1.2.1.11. Property `Setlist > presets > presets items > buttons > additionalProperties > radio_group`](#presets_items_buttons_additionalProperties_radio_group)
        - [1.1.2.1.12. Property `Setlist > presets > presets items > buttons > additionalProperties > renderer`](#presets_items_buttons_additionalProperties_renderer)
        - [1.1.2.1.13. Property `Setlist > presets > presets items > buttons > additionalProperties > renderer_param`](#presets_items_buttons_additionalProperties_renderer_param)
        - [1.1.2.1.14. Property `Setlist > presets > presets items > buttons > additionalProperties > reverse`](#presets_items_buttons_additionalProperties_reverse)
        - [1.1.2.1.15. Property `Setlist > presets > presets items > buttons > additionalProperties > toggle`](#presets_items_buttons_additionalProperties_toggle)
        - [1.1.2.1.16. Property `Setlist > presets > presets items > buttons > additionalProperties > value`](#presets_items_buttons_additionalProperties_value)
        - [1.1.2.1.17. Property `Setlist > presets > presets items > buttons > additionalProperties > values`](#presets_items_buttons_additionalProperties_values)
          - [1.1.2.1.17.1. Setlist > presets > presets items > buttons > additionalProperties > values > values items](#presets_items_buttons_additionalProperties_values_items)
    - [1.1.3. Property `Setlist > presets > presets items > defaults`](#presets_items_defaults)
      - [1.1.3.1. Property `Setlist > presets > presets items > defaults > anyOf > DefaultsConfig`](#presets_items_defaults_anyOf_i0)
        - [1.1.3.1.1. Property `Setlist > presets > presets items > defaults > anyOf > item 0 > buttons`](#presets_items_defaults_anyOf_i0_buttons)
          - [1.1.3.1.1.1. Property `Setlist > presets > presets items > defaults > anyOf > item 0 > buttons > additionalProperties`](#presets_items_defaults_anyOf_i0_buttons_additionalProperties)
        - [1.1.3.1.2. Property `Setlist > presets > presets items > defaults > anyOf > item 0 > encoders`](#presets_items_defaults_anyOf_i0_encoders)
          - [1.1.3.1.2.1. Property `Setlist > presets > presets items > defaults > anyOf > item 0 > encoders > additionalProperties`](#presets_items_defaults_anyOf_i0_encoders_additionalProperties)
      - [1.1.3.2. Property `Setlist > presets > presets items > defaults > anyOf > item 1`](#presets_items_defaults_anyOf_i1)
    - [1.1.4. Property `Setlist > presets > presets items > encoders`](#presets_items_encoders)
      - [1.1.4.1. Property `Setlist > presets > presets items > encoders > EncoderConfig`](#presets_items_encoders_additionalProperties)
        - [1.1.4.1.1. Property `Setlist > presets > presets items > encoders > additionalProperties > cc`](#presets_items_encoders_additionalProperties_cc)
        - [1.1.4.1.2. Property `Setlist > presets > presets items > encoders > additionalProperties > channel`](#presets_items_encoders_additionalProperties_channel)
        - [1.1.4.1.3. Property `Setlist > presets > presets items > encoders > additionalProperties > label`](#presets_items_encoders_additionalProperties_label)
    - [1.1.5. Property `Setlist > presets > presets items > name`](#presets_items_name)

**Title:** Setlist

|                           |                  |
| ------------------------- | ---------------- |
| **Type**                  | `object`         |
| **Required**              | No               |
| **Additional properties** | Any type allowed |

**Description:** A setlist file containing one or more presets.

| Property               | Pattern | Type  | Deprecated | Definition | Title/Description                                                                                         |
| ---------------------- | ------- | ----- | ---------- | ---------- | --------------------------------------------------------------------------------------------------------- |
| + [presets](#presets ) | No      | array | No         | -          | List of presets. Each preset defines the complete button/encoder/expression layout for one song or scene. |

## <a name="presets"></a>1. Property `Setlist > presets`

|              |         |
| ------------ | ------- |
| **Type**     | `array` |
| **Required** | Yes     |

**Description:** List of presets. Each preset defines the complete button/encoder/expression layout for one song or scene.

|                      | Array restrictions |
| -------------------- | ------------------ |
| **Min items**        | N/A                |
| **Max items**        | N/A                |
| **Items unicity**    | False              |
| **Additional items** | False              |
| **Tuple validation** | See below          |

| Each item of this array must be | Description                                          |
| ------------------------------- | ---------------------------------------------------- |
| [PresetConfig](#presets_items)  | A single preset — one song or scene in your setlist. |

### <a name="presets_items"></a>1.1. Setlist > presets > PresetConfig

|                           |                            |
| ------------------------- | -------------------------- |
| **Type**                  | `object`                   |
| **Required**              | No                         |
| **Additional properties** | Any type allowed           |
| **Defined in**            | #/definitions/PresetConfig |

**Description:** A single preset — one song or scene in your setlist.

| Property                               | Pattern | Type        | Deprecated | Definition | Title/Description                                                                                                 |
| -------------------------------------- | ------- | ----------- | ---------- | ---------- | ----------------------------------------------------------------------------------------------------------------- |
| - [analog](#presets_items_analog )     | No      | object      | No         | -          | Expression pedal configurations keyed by jack: Exp1, Exp2.                                                        |
| - [buttons](#presets_items_buttons )   | No      | object      | No         | -          | Button configurations keyed by position: A, B, C, D, E, F.                                                        |
| - [defaults](#presets_items_defaults ) | No      | Combination | No         | -          | Initial state on first activation after upload. Determines which toggles start ON and encoder starting positions. |
| - [encoders](#presets_items_encoders ) | No      | object      | No         | -          | Encoder configurations keyed by position: Vol (left), Gain (right).                                               |
| + [name](#presets_items_name )         | No      | string      | No         | -          | Preset name displayed on the OLED (max 16 characters).                                                            |

#### <a name="presets_items_analog"></a>1.1.1. Property `Setlist > presets > presets items > analog`

|                           |                                                                                                   |
| ------------------------- | ------------------------------------------------------------------------------------------------- |
| **Type**                  | `object`                                                                                          |
| **Required**              | No                                                                                                |
| **Additional properties** | [Each additional property must conform to the schema](#presets_items_analog_additionalProperties) |

**Description:** Expression pedal configurations keyed by jack: Exp1, Exp2.

| Property                                          | Pattern | Type   | Deprecated | Definition                        | Title/Description                              |
| ------------------------------------------------- | ------- | ------ | ---------- | --------------------------------- | ---------------------------------------------- |
| - [](#presets_items_analog_additionalProperties ) | No      | object | No         | In #/definitions/AnalogYamlConfig | Expression pedal (analog input) configuration. |

##### <a name="presets_items_analog_additionalProperties"></a>1.1.1.1. Property `Setlist > presets > presets items > analog > AnalogYamlConfig`

|                           |                                |
| ------------------------- | ------------------------------ |
| **Type**                  | `object`                       |
| **Required**              | No                             |
| **Additional properties** | Any type allowed               |
| **Defined in**            | #/definitions/AnalogYamlConfig |

**Description:** Expression pedal (analog input) configuration.

| Property                                                         | Pattern | Type            | Deprecated | Definition | Title/Description                               |
| ---------------------------------------------------------------- | ------- | --------------- | ---------- | ---------- | ----------------------------------------------- |
| + [cc](#presets_items_analog_additionalProperties_cc )           | No      | integer         | No         | -          | MIDI CC number to send (0-127).                 |
| - [channel](#presets_items_analog_additionalProperties_channel ) | No      | integer or null | No         | -          | MIDI channel (1-16). Default: 1.                |
| + [label](#presets_items_analog_additionalProperties_label )     | No      | string          | No         | -          | Display label for the expression pedal overlay. |
| - [max](#presets_items_analog_additionalProperties_max )         | No      | integer or null | No         | -          | Maximum CC value at toe position. Default: 127. |
| - [min](#presets_items_analog_additionalProperties_min )         | No      | integer or null | No         | -          | Minimum CC value at heel position. Default: 0.  |

###### <a name="presets_items_analog_additionalProperties_cc"></a>1.1.1.1.1. Property `Setlist > presets > presets items > analog > additionalProperties > cc`

|              |           |
| ------------ | --------- |
| **Type**     | `integer` |
| **Required** | Yes       |
| **Format**   | `uint8`   |

**Description:** MIDI CC number to send (0-127).

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_analog_additionalProperties_channel"></a>1.1.1.1.2. Property `Setlist > presets > presets items > analog > additionalProperties > channel`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |
| **Default**  | `null`            |

**Description:** MIDI channel (1-16). Default: 1.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_analog_additionalProperties_label"></a>1.1.1.1.3. Property `Setlist > presets > presets items > analog > additionalProperties > label`

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | Yes      |

**Description:** Display label for the expression pedal overlay.

###### <a name="presets_items_analog_additionalProperties_max"></a>1.1.1.1.4. Property `Setlist > presets > presets items > analog > additionalProperties > max`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |
| **Default**  | `null`            |

**Description:** Maximum CC value at toe position. Default: 127.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_analog_additionalProperties_min"></a>1.1.1.1.5. Property `Setlist > presets > presets items > analog > additionalProperties > min`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |
| **Default**  | `null`            |

**Description:** Minimum CC value at heel position. Default: 0.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

#### <a name="presets_items_buttons"></a>1.1.2. Property `Setlist > presets > presets items > buttons`

|                           |                                                                                                    |
| ------------------------- | -------------------------------------------------------------------------------------------------- |
| **Type**                  | `object`                                                                                           |
| **Required**              | No                                                                                                 |
| **Additional properties** | [Each additional property must conform to the schema](#presets_items_buttons_additionalProperties) |

**Description:** Button configurations keyed by position: A, B, C, D, E, F.

| Property                                           | Pattern | Type   | Deprecated | Definition                    | Title/Description                                                                            |
| -------------------------------------------------- | ------- | ------ | ---------- | ----------------------------- | -------------------------------------------------------------------------------------------- |
| - [](#presets_items_buttons_additionalProperties ) | No      | object | No         | In #/definitions/ButtonConfig | Button configuration. Use one of: note, cc, program_change, or actions for the MIDI message. |

##### <a name="presets_items_buttons_additionalProperties"></a>1.1.2.1. Property `Setlist > presets > presets items > buttons > ButtonConfig`

|                           |                            |
| ------------------------- | -------------------------- |
| **Type**                  | `object`                   |
| **Required**              | No                         |
| **Additional properties** | Any type allowed           |
| **Defined in**            | #/definitions/ButtonConfig |

**Description:** Button configuration. Use one of: note, cc, program_change, or actions for the MIDI message.

| Property                                                                        | Pattern | Type                     | Deprecated | Definition | Title/Description                                                                                                        |
| ------------------------------------------------------------------------------- | ------- | ------------------------ | ---------- | ---------- | ------------------------------------------------------------------------------------------------------------------------ |
| - [actions](#presets_items_buttons_additionalProperties_actions )               | No      | array or null            | No         | -          | Multi-action sequence: list of MIDI messages sent in order on press. Overrides cc/note/program_change fields.            |
| - [animation](#presets_items_buttons_additionalProperties_animation )           | No      | string or null           | No         | -          | LED animation when active. Values: solid, blink, pulse, rotate, colorcycle.                                              |
| - [cc](#presets_items_buttons_additionalProperties_cc )                         | No      | integer or null          | No         | -          | Send Control Change. Combined with toggle/values for different behaviors.                                                |
| - [channel](#presets_items_buttons_additionalProperties_channel )               | No      | integer or null          | No         | -          | MIDI channel (1-16). Default: 1. Applies to all actions on this button unless overridden per-action.                     |
| - [color](#presets_items_buttons_additionalProperties_color )                   | No      | string or null           | No         | -          | LED ring color when active. Values: red, green, blue, yellow, cyan, magenta, white, orange, purple, off, or #RRGGBB hex. |
| + [label](#presets_items_buttons_additionalProperties_label )                   | No      | string                   | No         | -          | Display label shown on OLED (max 16 characters).                                                                         |
| - [level](#presets_items_buttons_additionalProperties_level )                   | No      | boolean or null          | No         | -          | Level mode: LED brightness reflects CC value (for multi-LED visualization).                                              |
| - [note](#presets_items_buttons_additionalProperties_note )                     | No      | integer or null          | No         | -          | Send Note On/Off. Button press = Note On (velocity 127), release = Note Off.                                             |
| - [on_long_press](#presets_items_buttons_additionalProperties_on_long_press )   | No      | string or null           | No         | -          | Action on long press (hold > 500ms). Values: next_preset, prev_preset.                                                   |
| - [program_change](#presets_items_buttons_additionalProperties_program_change ) | No      | integer or null          | No         | -          | Send Program Change on press.                                                                                            |
| - [radio_group](#presets_items_buttons_additionalProperties_radio_group )       | No      | integer or null          | No         | -          | Radio group ID (0-255): only one button in the group can be active at a time. Pressing one deactivates others.           |
| - [renderer](#presets_items_buttons_additionalProperties_renderer )             | No      | string or null           | No         | -          | LED spatial renderer. Values: solid (all 12), fill (partial arc), single (one LED), dots (evenly-spaced).                |
| - [renderer_param](#presets_items_buttons_additionalProperties_renderer_param ) | No      | integer or null          | No         | -          | Renderer parameter: fill count (1-12), single position (0-11), or dot count (1-6).                                       |
| - [reverse](#presets_items_buttons_additionalProperties_reverse )               | No      | boolean or null          | No         | -          | Reverse cycle direction (cycle values list goes backward).                                                               |
| - [toggle](#presets_items_buttons_additionalProperties_toggle )                 | No      | boolean or null          | No         | -          | Toggle mode: alternates between on_press (active) and on_release (inactive) on each press. LED stays lit while active.   |
| - [value](#presets_items_buttons_additionalProperties_value )                   | No      | integer or null          | No         | -          | CC value to send (default: 127). For toggle mode, this is the ON value.                                                  |
| - [values](#presets_items_buttons_additionalProperties_values )                 | No      | array of integer or null | No         | -          | CC cycle values: each press sends the next value in the list. Use with cc field.                                         |

###### <a name="presets_items_buttons_additionalProperties_actions"></a>1.1.2.1.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions`

|              |                 |
| ------------ | --------------- |
| **Type**     | `array or null` |
| **Required** | No              |

**Description:** Multi-action sequence: list of MIDI messages sent in order on press. Overrides cc/note/program_change fields.

|                      | Array restrictions |
| -------------------- | ------------------ |
| **Min items**        | N/A                |
| **Max items**        | N/A                |
| **Items unicity**    | False              |
| **Additional items** | False              |
| **Tuple validation** | See below          |

| Each item of this array must be                                         | Description                                                             |
| ----------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| [ActionYaml](#presets_items_buttons_additionalProperties_actions_items) | A single action in a multi-action sequence. Exactly one type per entry. |

###### <a name="presets_items_buttons_additionalProperties_actions_items"></a>1.1.2.1.1.1. Setlist > presets > presets items > buttons > additionalProperties > actions > ActionYaml

|                           |                          |
| ------------------------- | ------------------------ |
| **Type**                  | `combining`              |
| **Required**              | No                       |
| **Additional properties** | Any type allowed         |
| **Defined in**            | #/definitions/ActionYaml |

**Description:** A single action in a multi-action sequence. Exactly one type per entry.

| Any of(Option)                                                               |
| ---------------------------------------------------------------------------- |
| [item 0](#presets_items_buttons_additionalProperties_actions_items_anyOf_i0) |
| [item 1](#presets_items_buttons_additionalProperties_actions_items_anyOf_i1) |
| [item 2](#presets_items_buttons_additionalProperties_actions_items_anyOf_i2) |
| [item 3](#presets_items_buttons_additionalProperties_actions_items_anyOf_i3) |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i0"></a>1.1.2.1.1.1.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 0`

|                           |                  |
| ------------------------- | ---------------- |
| **Type**                  | `object`         |
| **Required**              | No               |
| **Additional properties** | Any type allowed |

**Description:** Wait between actions (milliseconds).

| Property                                                                             | Pattern | Type    | Deprecated | Definition | Title/Description                             |
| ------------------------------------------------------------------------------------ | ------- | ------- | ---------- | ---------- | --------------------------------------------- |
| + [delay](#presets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay ) | No      | integer | No         | -          | Delay in milliseconds before the next action. |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i0_delay"></a>1.1.2.1.1.1.1.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 0 > delay`

|              |           |
| ------------ | --------- |
| **Type**     | `integer` |
| **Required** | Yes       |
| **Format**   | `uint16`  |

**Description:** Delay in milliseconds before the next action.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i1"></a>1.1.2.1.1.1.2. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 1`

|                           |                  |
| ------------------------- | ---------------- |
| **Type**                  | `object`         |
| **Required**              | No               |
| **Additional properties** | Any type allowed |

**Description:** Send a Control Change message.

| Property                                                                                 | Pattern | Type            | Deprecated | Definition | Title/Description                                     |
| ---------------------------------------------------------------------------------------- | ------- | --------------- | ---------- | ---------- | ----------------------------------------------------- |
| + [cc](#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc )           | No      | integer         | No         | -          | CC number (0-127).                                    |
| - [channel](#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel ) | No      | integer or null | No         | -          | MIDI channel (1-16). Inherits from button if omitted. |
| - [value](#presets_items_buttons_additionalProperties_actions_items_anyOf_i1_value )     | No      | integer or null | No         | -          | CC value (0-127). Default: 127.                       |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i1_cc"></a>1.1.2.1.1.1.2.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 1 > cc`

|              |           |
| ------------ | --------- |
| **Type**     | `integer` |
| **Required** | Yes       |
| **Format**   | `uint8`   |

**Description:** CC number (0-127).

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i1_channel"></a>1.1.2.1.1.1.2.2. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 1 > channel`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |

**Description:** MIDI channel (1-16). Inherits from button if omitted.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i1_value"></a>1.1.2.1.1.1.2.3. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 1 > value`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |

**Description:** CC value (0-127). Default: 127.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i2"></a>1.1.2.1.1.1.3. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 2`

|                           |                  |
| ------------------------- | ---------------- |
| **Type**                  | `object`         |
| **Required**              | No               |
| **Additional properties** | Any type allowed |

**Description:** Send a Program Change message.

| Property                                                                                               | Pattern | Type            | Deprecated | Definition | Title/Description                                     |
| ------------------------------------------------------------------------------------------------------ | ------- | --------------- | ---------- | ---------- | ----------------------------------------------------- |
| - [channel](#presets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel )               | No      | integer or null | No         | -          | MIDI channel (1-16). Inherits from button if omitted. |
| + [program_change](#presets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change ) | No      | integer         | No         | -          | Program number (0-127).                               |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i2_channel"></a>1.1.2.1.1.1.3.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 2 > channel`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |

**Description:** MIDI channel (1-16). Inherits from button if omitted.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i2_program_change"></a>1.1.2.1.1.1.3.2. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 2 > program_change`

|              |           |
| ------------ | --------- |
| **Type**     | `integer` |
| **Required** | Yes       |
| **Format**   | `uint8`   |

**Description:** Program number (0-127).

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i3"></a>1.1.2.1.1.1.4. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 3`

|                           |                  |
| ------------------------- | ---------------- |
| **Type**                  | `object`         |
| **Required**              | No               |
| **Additional properties** | Any type allowed |

**Description:** Send a Note On message (velocity 127).

| Property                                                                                 | Pattern | Type            | Deprecated | Definition | Title/Description                                     |
| ---------------------------------------------------------------------------------------- | ------- | --------------- | ---------- | ---------- | ----------------------------------------------------- |
| - [channel](#presets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel ) | No      | integer or null | No         | -          | MIDI channel (1-16). Inherits from button if omitted. |
| + [note](#presets_items_buttons_additionalProperties_actions_items_anyOf_i3_note )       | No      | integer         | No         | -          | MIDI note number (0-127).                             |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i3_channel"></a>1.1.2.1.1.1.4.1. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 3 > channel`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |

**Description:** MIDI channel (1-16). Inherits from button if omitted.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_actions_items_anyOf_i3_note"></a>1.1.2.1.1.1.4.2. Property `Setlist > presets > presets items > buttons > additionalProperties > actions > actions items > anyOf > item 3 > note`

|              |           |
| ------------ | --------- |
| **Type**     | `integer` |
| **Required** | Yes       |
| **Format**   | `uint8`   |

**Description:** MIDI note number (0-127).

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_animation"></a>1.1.2.1.2. Property `Setlist > presets > presets items > buttons > additionalProperties > animation`

|              |                  |
| ------------ | ---------------- |
| **Type**     | `string or null` |
| **Required** | No               |
| **Default**  | `null`           |

**Description:** LED animation when active. Values: solid, blink, pulse, rotate, colorcycle.

###### <a name="presets_items_buttons_additionalProperties_cc"></a>1.1.2.1.3. Property `Setlist > presets > presets items > buttons > additionalProperties > cc`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |
| **Default**  | `null`            |

**Description:** Send Control Change. Combined with toggle/values for different behaviors.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_channel"></a>1.1.2.1.4. Property `Setlist > presets > presets items > buttons > additionalProperties > channel`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |
| **Default**  | `null`            |

**Description:** MIDI channel (1-16). Default: 1. Applies to all actions on this button unless overridden per-action.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_color"></a>1.1.2.1.5. Property `Setlist > presets > presets items > buttons > additionalProperties > color`

|              |                  |
| ------------ | ---------------- |
| **Type**     | `string or null` |
| **Required** | No               |
| **Default**  | `null`           |

**Description:** LED ring color when active. Values: red, green, blue, yellow, cyan, magenta, white, orange, purple, off, or #RRGGBB hex.

###### <a name="presets_items_buttons_additionalProperties_label"></a>1.1.2.1.6. Property `Setlist > presets > presets items > buttons > additionalProperties > label`

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | Yes      |

**Description:** Display label shown on OLED (max 16 characters).

###### <a name="presets_items_buttons_additionalProperties_level"></a>1.1.2.1.7. Property `Setlist > presets > presets items > buttons > additionalProperties > level`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `boolean or null` |
| **Required** | No                |
| **Default**  | `null`            |

**Description:** Level mode: LED brightness reflects CC value (for multi-LED visualization).

###### <a name="presets_items_buttons_additionalProperties_note"></a>1.1.2.1.8. Property `Setlist > presets > presets items > buttons > additionalProperties > note`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |
| **Default**  | `null`            |

**Description:** Send Note On/Off. Button press = Note On (velocity 127), release = Note Off.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_on_long_press"></a>1.1.2.1.9. Property `Setlist > presets > presets items > buttons > additionalProperties > on_long_press`

|              |                  |
| ------------ | ---------------- |
| **Type**     | `string or null` |
| **Required** | No               |
| **Default**  | `null`           |

**Description:** Action on long press (hold > 500ms). Values: next_preset, prev_preset.

###### <a name="presets_items_buttons_additionalProperties_program_change"></a>1.1.2.1.10. Property `Setlist > presets > presets items > buttons > additionalProperties > program_change`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |
| **Default**  | `null`            |

**Description:** Send Program Change on press.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_radio_group"></a>1.1.2.1.11. Property `Setlist > presets > presets items > buttons > additionalProperties > radio_group`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |
| **Default**  | `null`            |

**Description:** Radio group ID (0-255): only one button in the group can be active at a time. Pressing one deactivates others.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_renderer"></a>1.1.2.1.12. Property `Setlist > presets > presets items > buttons > additionalProperties > renderer`

|              |                  |
| ------------ | ---------------- |
| **Type**     | `string or null` |
| **Required** | No               |
| **Default**  | `null`           |

**Description:** LED spatial renderer. Values: solid (all 12), fill (partial arc), single (one LED), dots (evenly-spaced).

###### <a name="presets_items_buttons_additionalProperties_renderer_param"></a>1.1.2.1.13. Property `Setlist > presets > presets items > buttons > additionalProperties > renderer_param`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |
| **Default**  | `null`            |

**Description:** Renderer parameter: fill count (1-12), single position (0-11), or dot count (1-6).

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_reverse"></a>1.1.2.1.14. Property `Setlist > presets > presets items > buttons > additionalProperties > reverse`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `boolean or null` |
| **Required** | No                |
| **Default**  | `null`            |

**Description:** Reverse cycle direction (cycle values list goes backward).

###### <a name="presets_items_buttons_additionalProperties_toggle"></a>1.1.2.1.15. Property `Setlist > presets > presets items > buttons > additionalProperties > toggle`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `boolean or null` |
| **Required** | No                |
| **Default**  | `null`            |

**Description:** Toggle mode: alternates between on_press (active) and on_release (inactive) on each press. LED stays lit while active.

###### <a name="presets_items_buttons_additionalProperties_value"></a>1.1.2.1.16. Property `Setlist > presets > presets items > buttons > additionalProperties > value`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |
| **Default**  | `null`            |

**Description:** CC value to send (default: 127). For toggle mode, this is the ON value.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_buttons_additionalProperties_values"></a>1.1.2.1.17. Property `Setlist > presets > presets items > buttons > additionalProperties > values`

|              |                            |
| ------------ | -------------------------- |
| **Type**     | `array of integer or null` |
| **Required** | No                         |
| **Default**  | `null`                     |

**Description:** CC cycle values: each press sends the next value in the list. Use with cc field.

|                      | Array restrictions |
| -------------------- | ------------------ |
| **Min items**        | N/A                |
| **Max items**        | N/A                |
| **Items unicity**    | False              |
| **Additional items** | False              |
| **Tuple validation** | See below          |

| Each item of this array must be                                          | Description |
| ------------------------------------------------------------------------ | ----------- |
| [values items](#presets_items_buttons_additionalProperties_values_items) | -           |

###### <a name="presets_items_buttons_additionalProperties_values_items"></a>1.1.2.1.17.1. Setlist > presets > presets items > buttons > additionalProperties > values > values items

|              |           |
| ------------ | --------- |
| **Type**     | `integer` |
| **Required** | No        |
| **Format**   | `uint8`   |

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

#### <a name="presets_items_defaults"></a>1.1.3. Property `Setlist > presets > presets items > defaults`

|                           |                  |
| ------------------------- | ---------------- |
| **Type**                  | `combining`      |
| **Required**              | No               |
| **Additional properties** | Any type allowed |

**Description:** Initial state on first activation after upload. Determines which toggles start ON and encoder starting positions.

| Any of(Option)                                     |
| -------------------------------------------------- |
| [DefaultsConfig](#presets_items_defaults_anyOf_i0) |
| [item 1](#presets_items_defaults_anyOf_i1)         |

##### <a name="presets_items_defaults_anyOf_i0"></a>1.1.3.1. Property `Setlist > presets > presets items > defaults > anyOf > DefaultsConfig`

|                           |                              |
| ------------------------- | ---------------------------- |
| **Type**                  | `object`                     |
| **Required**              | No                           |
| **Additional properties** | Any type allowed             |
| **Defined in**            | #/definitions/DefaultsConfig |

**Description:** Default initial state for a preset on first activation after upload.

| Property                                                 | Pattern | Type   | Deprecated | Definition | Title/Description                                                                        |
| -------------------------------------------------------- | ------- | ------ | ---------- | ---------- | ---------------------------------------------------------------------------------------- |
| - [buttons](#presets_items_defaults_anyOf_i0_buttons )   | No      | object | No         | -          | Button keys (A-F) mapped to "on" or "off". Omitted buttons default to off.               |
| - [encoders](#presets_items_defaults_anyOf_i0_encoders ) | No      | object | No         | -          | Encoder keys (Vol, Gain) mapped to initial value (0-127). Omitted encoders default to 0. |

###### <a name="presets_items_defaults_anyOf_i0_buttons"></a>1.1.3.1.1. Property `Setlist > presets > presets items > defaults > anyOf > item 0 > buttons`

|                           |                                                                                                                      |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| **Type**                  | `object`                                                                                                             |
| **Required**              | No                                                                                                                   |
| **Additional properties** | [Each additional property must conform to the schema](#presets_items_defaults_anyOf_i0_buttons_additionalProperties) |
| **Default**               | `{}`                                                                                                                 |

**Description:** Button keys (A-F) mapped to "on" or "off". Omitted buttons default to off.

| Property                                                             | Pattern | Type   | Deprecated | Definition | Title/Description |
| -------------------------------------------------------------------- | ------- | ------ | ---------- | ---------- | ----------------- |
| - [](#presets_items_defaults_anyOf_i0_buttons_additionalProperties ) | No      | string | No         | -          | -                 |

###### <a name="presets_items_defaults_anyOf_i0_buttons_additionalProperties"></a>1.1.3.1.1.1. Property `Setlist > presets > presets items > defaults > anyOf > item 0 > buttons > additionalProperties`

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | No       |

###### <a name="presets_items_defaults_anyOf_i0_encoders"></a>1.1.3.1.2. Property `Setlist > presets > presets items > defaults > anyOf > item 0 > encoders`

|                           |                                                                                                                       |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| **Type**                  | `object`                                                                                                              |
| **Required**              | No                                                                                                                    |
| **Additional properties** | [Each additional property must conform to the schema](#presets_items_defaults_anyOf_i0_encoders_additionalProperties) |
| **Default**               | `{}`                                                                                                                  |

**Description:** Encoder keys (Vol, Gain) mapped to initial value (0-127). Omitted encoders default to 0.

| Property                                                              | Pattern | Type    | Deprecated | Definition | Title/Description |
| --------------------------------------------------------------------- | ------- | ------- | ---------- | ---------- | ----------------- |
| - [](#presets_items_defaults_anyOf_i0_encoders_additionalProperties ) | No      | integer | No         | -          | -                 |

###### <a name="presets_items_defaults_anyOf_i0_encoders_additionalProperties"></a>1.1.3.1.2.1. Property `Setlist > presets > presets items > defaults > anyOf > item 0 > encoders > additionalProperties`

|              |           |
| ------------ | --------- |
| **Type**     | `integer` |
| **Required** | No        |
| **Format**   | `uint8`   |

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

##### <a name="presets_items_defaults_anyOf_i1"></a>1.1.3.2. Property `Setlist > presets > presets items > defaults > anyOf > item 1`

|              |        |
| ------------ | ------ |
| **Type**     | `null` |
| **Required** | No     |

#### <a name="presets_items_encoders"></a>1.1.4. Property `Setlist > presets > presets items > encoders`

|                           |                                                                                                     |
| ------------------------- | --------------------------------------------------------------------------------------------------- |
| **Type**                  | `object`                                                                                            |
| **Required**              | No                                                                                                  |
| **Additional properties** | [Each additional property must conform to the schema](#presets_items_encoders_additionalProperties) |

**Description:** Encoder configurations keyed by position: Vol (left), Gain (right).

| Property                                            | Pattern | Type   | Deprecated | Definition                     | Title/Description             |
| --------------------------------------------------- | ------- | ------ | ---------- | ------------------------------ | ----------------------------- |
| - [](#presets_items_encoders_additionalProperties ) | No      | object | No         | In #/definitions/EncoderConfig | Rotary encoder configuration. |

##### <a name="presets_items_encoders_additionalProperties"></a>1.1.4.1. Property `Setlist > presets > presets items > encoders > EncoderConfig`

|                           |                             |
| ------------------------- | --------------------------- |
| **Type**                  | `object`                    |
| **Required**              | No                          |
| **Additional properties** | Any type allowed            |
| **Defined in**            | #/definitions/EncoderConfig |

**Description:** Rotary encoder configuration.

| Property                                                           | Pattern | Type            | Deprecated | Definition | Title/Description                                                                  |
| ------------------------------------------------------------------ | ------- | --------------- | ---------- | ---------- | ---------------------------------------------------------------------------------- |
| - [cc](#presets_items_encoders_additionalProperties_cc )           | No      | integer or null | No         | -          | MIDI CC number to send (0-127). Each detent click increments/decrements the value. |
| - [channel](#presets_items_encoders_additionalProperties_channel ) | No      | integer or null | No         | -          | MIDI channel (1-16). Default: 1.                                                   |
| + [label](#presets_items_encoders_additionalProperties_label )     | No      | string          | No         | -          | Display label shown on OLED overlay when turning.                                  |

###### <a name="presets_items_encoders_additionalProperties_cc"></a>1.1.4.1.1. Property `Setlist > presets > presets items > encoders > additionalProperties > cc`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint16`          |
| **Default**  | `null`            |

**Description:** MIDI CC number to send (0-127). Each detent click increments/decrements the value.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_encoders_additionalProperties_channel"></a>1.1.4.1.2. Property `Setlist > presets > presets items > encoders > additionalProperties > channel`

|              |                   |
| ------------ | ----------------- |
| **Type**     | `integer or null` |
| **Required** | No                |
| **Format**   | `uint8`           |
| **Default**  | `null`            |

**Description:** MIDI channel (1-16). Default: 1.

| Restrictions |     |
| ------------ | --- |
| **Minimum**  | N/A |

###### <a name="presets_items_encoders_additionalProperties_label"></a>1.1.4.1.3. Property `Setlist > presets > presets items > encoders > additionalProperties > label`

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | Yes      |

**Description:** Display label shown on OLED overlay when turning.

#### <a name="presets_items_name"></a>1.1.5. Property `Setlist > presets > presets items > name`

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | Yes      |

**Description:** Preset name displayed on the OLED (max 16 characters).

----------------------------------------------------------------------------------------------------------------------------
Generated using [json-schema-for-humans](https://github.com/coveooss/json-schema-for-humans) on 2026-06-29 at 00:29:28 +0200
