use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub value: Option<String>,
}

/**
Represents the info of a bluetooth device.
Typescript reference:

```typescript
interface DeviceInfo {
  id: string
  name?: string
  services: string[]
  rssi?: number
}
```
*/
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub services: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rssi: Option<i16>,
}
