import { isTauri } from '@tauri-apps/api/core'

import { BluetoothDevice, DeviceInfo, RequestDeviceTauriOptions } from './types'
import { tauriInvoke } from './utils'

export const ping = async (value: string): Promise<string | null> =>
  await tauriInvoke<{
    value?: string
  }>('ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null))

export const getAvailability = async (): Promise<boolean> => {
  if (isTauri()) {
    return await tauriInvoke<boolean>('get_availability')
  } else {
    return (await navigator.bluetooth?.getAvailability()) ?? false
  }
}

export const requestDevice = async (
  options: RequestDeviceOptions & RequestDeviceTauriOptions,
): Promise<BluetoothDevice | undefined> => {
  if (!(await getAvailability())) {
    return
  }

  if (isTauri()) {
    const info = await tauriInvoke<DeviceInfo>('request_device', { options })
    console.log(info)
    return new BluetoothDevice(info.id, info.name)
  } else {
    const device = await navigator.bluetooth.requestDevice(options)
    console.log(device)
    return new BluetoothDevice(device)
  }
}

export const scanDevices = async (
  options: RequestDeviceOptions & RequestDeviceTauriOptions,
): Promise<DeviceInfo[]> => {
  if (!(await getAvailability())) {
    return []
  }

  if (isTauri()) {
    const devices = await tauriInvoke<DeviceInfo[]>('scan_devices', { options })
    return devices
  } else {
    throw new Error('scanDevices is only available in Tauri environment')
  }
}
