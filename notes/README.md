# Dev Notes

...

## Potential dependencies:

* Waw decoding:
    + [hound](https://crates.io/crates/hound)
    + [ffmpeg libavcodec][]
* Video (proprietary "Blink") decoding:
    + [ffmpeg libavcodec][] - [there][bink-reversed] format reverced
* iOS native UI:
    + [Cocoa UIKit bindings](https://github.com/SSheldon/rust-uikit)

[ffmpeg libavcodec]:https://git.ffmpeg.org/gitweb/ffmpeg.git/tree/HEAD:/libavcodec
[bink-reversed]:http://article.gmane.org/gmane.comp.video.ffmpeg.cvs/28414


## iOS

Run tests on iOS devices - use [Dinghy crate / Cargo plugin][Dinghy-github].

[Dinghy-crate]:https://crates.io/crates/dinghy
[Dinghy-github]:https://github.com/snipsco/dinghy
[Dinghy-post]:https://medium.com/snips-ai/dinghy-painless-rust-tests-and-benches-on-ios-and-android-c9f94f81d305

Read more: [crate][Dinghy-crate], [github][Dinghy-github], [post][Dinghy-post].

Install onto iPhone 4 (iOS 7) - [ios-deploy](https://github.com/phonegap/ios-deploy).

> __Error Codes__: [table](https://www.theiphonewiki.com/wiki/MobileDevice_Library#Known_Error_Codes), [src](https://pewpewthespells.com/media/MobileDevice.h).

> __Add DeviceSupport to Xcode__: [SO](https://stackoverflow.com/a/39655973/829264) - download [image](https://drive.google.com/open?id=0B3AdrmeePh3MRlU2bUphYXlBa1E) and put to xcode `Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/DeviceSupport/7.1/` . ([tool maybe can help](https://github.com/KrauseFx/xcode-install))

[Debug on iOS7](http://iphonedevwiki.net/index.php/Debugging_on_iOS_7) [__with lldb__](http://codedigging.com/blog/2016-04-27-debugging-ios-binaries-with-lldb/).

