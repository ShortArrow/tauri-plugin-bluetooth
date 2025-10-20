package org.studio26f.tauri.plugin.bluetooth

import android.Manifest
import android.app.Activity
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanResult
import android.content.pm.PackageManager
import android.os.Build
import android.os.Handler
import android.os.Looper
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.Permission
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class PingArgs {
  var value: String? = null
}

@InvokeArg
class ScanOptions {
  var acceptAllDevices: Boolean? = true
  var timeout: Long? = 5000
}

@TauriPlugin(
    permissions = [
        Permission(strings = [Manifest.permission.BLUETOOTH_SCAN], alias = "bluetoothScan"),
        Permission(strings = [Manifest.permission.BLUETOOTH_CONNECT], alias = "bluetoothConnect"),
        Permission(strings = [Manifest.permission.ACCESS_FINE_LOCATION], alias = "location")
    ]
)
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = Example()
    private val bluetoothAdapter: BluetoothAdapter? by lazy {
        val bluetoothManager = activity.getSystemService(Activity.BLUETOOTH_SERVICE) as? BluetoothManager
        bluetoothManager?.adapter
    }
    private val scannedDevices = mutableListOf<ScanResult>()
    private val handler = Handler(Looper.getMainLooper())

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)
        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }

    @Command
    fun scanDevices(invoke: Invoke) {
        android.util.Log.i("ExamplePlugin", "scanDevices called")

        // 権限チェック
        if (!checkBluetoothPermissions()) {
            android.util.Log.w("ExamplePlugin", "Permissions not granted, requesting...")
            requestBluetoothPermissions(invoke)
            return
        }

        val args = invoke.parseArgs(ScanOptions::class.java)
        val timeout = args.timeout ?: 5000L

        scannedDevices.clear()

        val scanner = bluetoothAdapter?.bluetoothLeScanner
        if (scanner == null) {
            invoke.reject("Bluetooth not available")
            return
        }

        val scanCallback = object : ScanCallback() {
            override fun onScanResult(callbackType: Int, result: ScanResult) {
                super.onScanResult(callbackType, result)
                scannedDevices.add(result)
                android.util.Log.d("ExamplePlugin", "Found device: ${result.device.name ?: "Unknown"}, RSSI: ${result.rssi}")
            }

            override fun onScanFailed(errorCode: Int) {
                super.onScanFailed(errorCode)
                android.util.Log.e("ExamplePlugin", "Scan failed with error: $errorCode")
            }
        }

        try {
            scanner.startScan(scanCallback)
            android.util.Log.i("ExamplePlugin", "Scan started, timeout: ${timeout}ms")

            handler.postDelayed({
                scanner.stopScan(scanCallback)
                android.util.Log.i("ExamplePlugin", "Scan stopped, found ${scannedDevices.size} devices")

                val devicesArray = JSArray()
                scannedDevices.forEach { result ->
                    val device = JSObject()
                    device.put("id", result.device.address)
                    device.put("name", result.device.name ?: "Unknown")
                    device.put("rssi", result.rssi)
                    device.put("services", JSArray())
                    devicesArray.put(device)
                }

                val ret = JSObject()
                ret.put("devices", devicesArray)
                invoke.resolve(ret)
            }, timeout)
        } catch (e: SecurityException) {
            android.util.Log.e("ExamplePlugin", "Security exception during scan", e)
            invoke.reject("Permission denied: ${e.message}")
        }
    }

    private fun checkBluetoothPermissions(): Boolean {
        val permissions = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            arrayOf(
                Manifest.permission.BLUETOOTH_SCAN,
                Manifest.permission.BLUETOOTH_CONNECT,
                Manifest.permission.ACCESS_FINE_LOCATION
            )
        } else {
            arrayOf(
                Manifest.permission.ACCESS_FINE_LOCATION
            )
        }

        return permissions.all {
            ContextCompat.checkSelfPermission(activity, it) == PackageManager.PERMISSION_GRANTED
        }
    }

    private fun requestBluetoothPermissions(invoke: Invoke) {
        val permissions = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            arrayOf(
                Manifest.permission.BLUETOOTH_SCAN,
                Manifest.permission.BLUETOOTH_CONNECT,
                Manifest.permission.ACCESS_FINE_LOCATION
            )
        } else {
            arrayOf(
                Manifest.permission.ACCESS_FINE_LOCATION
            )
        }

        ActivityCompat.requestPermissions(activity, permissions, 1001)
        invoke.reject("Permissions required. Please grant Bluetooth and Location permissions and try again.")
    }
}
