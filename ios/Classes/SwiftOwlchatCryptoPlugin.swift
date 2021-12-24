import Flutter
import UIKit

public class SwiftOwlchatCryptoPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "owlchat_crypto", binaryMessenger: registrar.messenger())
    let instance = SwiftOwlchatCryptoPlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    result("iOS " + UIDevice.current.systemVersion)
  }

  public static func dummyMethodToEnforceBundling() {
      owlchat_crypto_keypair_new()
  }
}
