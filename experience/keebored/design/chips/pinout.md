# Pinout

| Net           | Pin         | Functions    | Notes                                                         |
| ------------- | ----------- | ------------ | ------------------------------------------------------------- |
| LED           | 2           | PIO0         | Higher priority PIO, consider using another pin for proximity |
| VBUS_DET      | 3           | USB_VBUS_DET | Low priority (just needs to know if VBUS exists)              |
| 3V3           | 10,22,33,49 | IOVDD        | Power,                                                        |
| ROTARY_A      | 11          |              | Low priority                                                  |
| ROTARY_B      | 12          |              | Low priority                                                  |
| ROTARY_PUSH   | 14          |              | Lowest priority lol                                           |
|               |             |              |                                                               |
| ADC0          | 38          | ADC          | HIGH PRIORITY                                                 |
| ADC1          | 39          | ADC          | HIGH PRIORITY                                                 |
| ADC2          | 40          | ADC          | HIGH PRIORITY                                                 |
| ADC3          | 41          | ADC          | HIGH PRIORITY                                                 |
| No Connection |             |              |                                                               |

