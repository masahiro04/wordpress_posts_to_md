
FlutterにFirebase追加したら下記のエラーが発生しました

※一部だけ掲載予定でしたが、思いのほかハマったので後続の方のために全て記載しています

バージョン情報

```dart
 flutter: 3.0.5
Dart: 2.18 ?ぐらい、ここあんま関係ない
Mac: M1 MAX
```

```swift
 Changing current working directory to: /Users/masahirookubo/me/flutter/firestore_test
Launching lib/main.dart on iPad Pro (12.9-inch) (5th generation) in debug mode...
Running pod install...                                             16.4s
Running Xcode build...
Xcode build done.                                           54.7s
Failed to build iOS app
Error output from Xcode build:
↳
    objc[54809]: Class AMSupportURLConnectionDelegate is implemented in both /usr/lib/libauthinstall.dylib (0x1ddd5eb90) and /Library/Apple/System/Library/PrivateFrameworks/MobileDevice.framework/Versions/A/MobileDevice (0x103b382c8). One of the two will be used. Which one is undefined.
    objc[54809]: Class AMSupportURLSession is implemented in both /usr/lib/libauthinstall.dylib (0x1ddd5ebe0) and /Library/Apple/System/Library/PrivateFrameworks/MobileDevice.framework/Versions/A/MobileDevice (0x103b38318). One of the two will be used. Which one is undefined.
    ** BUILD FAILED **


Xcode's output:
↳
    Writing result bundle at path:
        /var/folders/bb/n3dq0s2963v_4c8dw8ygjlgm0000gn/T/flutter_tools.IPaDUI/flutter_ios_build_temp_dirGu70oL/temporary_xcresult_bundle

    While building module 'absl' imported from /Users/masahirookubo/me/flutter/firestore_test/ios/Pods/gRPC-Core/src/core/ext/xds/xds_server_config_fetcher.cc:21:
    In file included from <module-includes>:1:
    In file included from /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/abseil-umbrella.h:13:
    In file included from /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/algorithm/algorithm.h:29:
    /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/base/config.h:52:10: error: include of non-modular header inside framework module 'absl.base.config': '/Library/Developer/Toolchains/swift-5.6.2-RELEASE.xctoolchain/usr/bin/../include/c++/v1/limits.h' [-Werror,-Wnon-modular-include-in-framework-module]
    #include <limits.h>
             ^
    While building module 'absl' imported from /Users/masahirookubo/me/flutter/firestore_test/ios/Pods/gRPC-Core/src/core/ext/xds/xds_server_config_fetcher.cc:21:
    In file included from <module-includes>:1:
    In file included from /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/abseil-umbrella.h:13:
    In file included from /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/algorithm/algorithm.h:29:
    In file included from /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/base/config.h:67:
    /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/base/policy_checks.h:28:10: error: include of non-modular header inside framework module 'absl.base.policy_checks': '/Library/Developer/Toolchains/swift-5.6.2-RELEASE.xctoolchain/usr/bin/../include/c++/v1/limits.h' [-Werror,-Wnon-modular-include-in-framework-module]
    #include <limits.h>
             ^
    While building module 'absl' imported from /Users/masahirookubo/me/flutter/firestore_test/ios/Pods/gRPC-Core/src/core/ext/xds/xds_server_config_fetcher.cc:21:
    In file included from <module-includes>:1:
    In file included from /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/abseil-umbrella.h:53:
    /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/base/internal/spinlock_linux.inc:17:10: fatal error: 'linux/futex.h' file not found
    #include <linux/futex.h>
             ^~~~~~~~~~~~~~~
    3 errors generated.
    /Users/masahirookubo/me/flutter/firestore_test/ios/Pods/gRPC-Core/src/core/ext/xds/xds_server_config_fetcher.cc:21:10: fatal error: could not build module 'absl'
    #include "absl/strings/str_join.h"
     ~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~
    While building module 'openssl' imported from /Users/masahirookubo/me/flutter/firestore_test/ios/Pods/gRPC-Core/src/core/tsi/ssl_transport_security.h:25:
    In file included from <module-includes>:1:
    /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/BoringSSL-GRPC/openssl_grpc.framework/Headers/umbrella.h:39:1: warning: umbrella header for module 'openssl' does not include header 'siphash.h' [-Wincomplete-umbrella]
    /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/BoringSSL-GRPC/openssl_grpc.framework/Headers/umbrella.h:39:1: warning: umbrella header for module 'openssl' does not include header 'x509_vfy.h' [-Wincomplete-umbrella]
    /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/BoringSSL-GRPC/openssl_grpc.framework/Headers/umbrella.h:39:1: warning: umbrella header for module 'openssl' does not include header 'hpke.h' [-Wincomplete-umbrella]
    /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/BoringSSL-GRPC/openssl_grpc.framework/Headers/umbrella.h:39:1: warning: umbrella header for module 'openssl' does not include header 'e_os2.h' [-Wincomplete-umbrella]
    /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/BoringSSL-GRPC/openssl_grpc.framework/Headers/umbrella.h:39:1: warning: umbrella header for module 'openssl' does not include header 'blake2.h' [-Wincomplete-umbrella]
    /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/BoringSSL-GRPC/openssl_grpc.framework/Headers/umbrella.h:39:1: warning: umbrella header for module 'openssl' does not include header 'hrss.h' [-Wincomplete-umbrella]
    /Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/BoringSSL-GRPC/openssl_grpc.framework/Headers/umbrella.h:39:1: warning: umbrella header for module 'openssl' does not include header 'trust_token.h' [-Wincomplete-umbrella]
    7 warnings generated.
    7 warnings and 4 errors generated.
    note: Using new build system
    note: Planning
    note: Build preparation complete
    note: Building targets in dependency order
    /Users/masahirookubo/me/flutter/firestore_test/ios/Pods/Pods.xcodeproj: warning: The iOS Simulator deployment target 'IPHONEOS_DEPLOYMENT_TARGET' is set to 8.0, but the range of supported deployment target versions is 9.0 to 15.2.99. (in target 'leveldb-library' from project 'Pods')

    Result bundle written to path:
        /var/folders/bb/n3dq0s2963v_4c8dw8ygjlgm0000gn/T/flutter_tools.IPaDUI/flutter_ios_build_temp_dirGu70oL/temporary_xcresult_bundle


Lexical or Preprocessor Issue (Xcode): Include of non-modular header inside framework module 'absl.base.config': '/Library/Developer/Toolchains/swift-5.6.2-RELEASE.xctoolchain/usr/bin/../include/c++/v1/limits.h'
/Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/base/config.h:51:9

Lexical or Preprocessor Issue (Xcode): Include of non-modular header inside framework module 'absl.base.policy_checks': '/Library/Developer/Toolchains/swift-5.6.2-RELEASE.xctoolchain/usr/bin/../include/c++/v1/limits.h'
/Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/base/policy_checks.h:27:9

Lexical or Preprocessor Issue (Xcode): 'linux/futex.h' file not found
/Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/base/internal/spinlock_linux.inc:16:9

Parse Issue (Xcode): Could not build module 'absl'
/Users/masahirookubo/me/flutter/firestore_test/ios/Pods/gRPC-Core/src/core/ext/xds/xds_server_config_fetcher.cc:20:9

Could not build the application for the simulator.
Error launching application on iPad Pro (12.9-inch) (5th generation).

[masahirookubo@MacBook-Pro-2 (arm64):~/me/flutter/firestore_test/ios]!+[master]
%
```



## 結論

XCodeを再インストールしました

下記のようなコマンドも実行しましたが特に変わらず

```swift
 cd ios
rm -rf ~/Library/Caches/CocoaPods
rm -rf Podfile.lock
rm -rf Pods
rm -rf ~/Library/Developer/Xcode/DerivedData/*
```

下記の手順でXCodeを削除、そしてinstallで解決できました

```dart
 sudo rm -rf /Applications/Xcode.app
sudo rm -rf /Library/Preferences/com.apple.dt.Xcode.plist
sudo rm -rf ~/Library/Preferences/com.apple.dt.Xcode.plist
sudo rm -rf ~/Library/Caches/com.apple.dt.Xcode
sudo rm -rf ~/Library/Application Support/Xcode
sudo rm -rf ~/Library/Developer/Xcode
sudo rm -rf ~/Library/Developer/CoreSimulator

// こちらは念の為実行、理由は後述
sudo rm -rf /Library/Developer/CommandLineTools
```

## 問題の原因

再インストール直前で、2点が目につきました

```swift
  One of the two will be used. Which one is undefined

Lexical or Preprocessor Issue (Xcode): Include of non-modular header inside framework module 'absl.base.config': '/Library/Developer/Toolchains/swift-5.6.2-RELEASE.xctoolchain/usr/bin/../include/c++/v1/limits.h'
/Users/masahirookubo/me/flutter/firestore_test/build/ios/Debug-iphonesimulator/abseil/absl.framework/Headers/base/config.h:51:9
```

1行目のエラー情報は割とあるあるだと思うのですが、下の方の情報にはswift-5.6.2がToolchainsから参照されています

ここは思いっきり心当たりがあって、1ヶ月ほど前にvimでswift開発をするためにToolchainsやswiftバージョンをを変えるために、必要なコードをダウンロードしてきたものです

途中までは流石に問題を引き起こすことはないだろう、ぐらいに考えていたのですが、

- Flutterアプリをcreate
- flirebaseを導入
- cloudstore導入で急にエラー



はあまりにもおかし過ぎるし、もしこれが普遍的な問題であれば社会的な問題にもつながると思います

※大きなインフラなので冗談抜きで本当に問題として取り上げられ、twitterなどですぐに情報が回るはず



なので一度削除する、という選択に踏み切りました

## 注意点

再インストール自体は簡単な選択ですが、

- 時間かかる(2, 3時間ぐらいかかった。通信状況などにも大きく依存するかと)
- アカウント登録情報も削除される(Apple IDでログインすれば良いだけだけど、手元に情報を用意しておく必要あり)
- もしかしたら、重要なファイルや情報なども削除されるかも
- Toolchainsを削除するとgitなど関連するものを全て削除されるので、削除前によく検討すること

が必要です

自分の場合は基本的にDockerで環境を整えている、かつSwift、Dartでの開発しか行っていないのですぐに踏み切れましたが、複数の案件に関わっている場合や、時間がない場合は特に注意が必要です

## まとめ

おそらく上記の順番で実行すれば、cloudfireだけでなくほとんどの問題解決の最終手段になり得ると思います

調べてもダメな場合は、寝る前に再インストールして朝実行ということも可能かと思うので、困っている方は最後の手段としておすすめです🙏

## 参考記事

[Xcodeの再インストール](https://qiita.com/___fff_/items/0815915fad807fcbd546)




