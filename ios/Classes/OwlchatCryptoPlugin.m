#import "OwlchatCryptoPlugin.h"
#if __has_include(<owlchat_crypto/owlchat_crypto-Swift.h>)
#import <owlchat_crypto/owlchat_crypto-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "owlchat_crypto-Swift.h"
#endif

@implementation OwlchatCryptoPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftOwlchatCryptoPlugin registerWithRegistrar:registrar];
}
@end
