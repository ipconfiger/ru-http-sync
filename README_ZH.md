# ru-http-sync
局域网快速收发文件的小工具

[English README](./README.md)

## 说明

在局域网中要分享大文件给朋友，但是又不希望通过聊天工具从公网转发一次（会很慢）的话，就可以使用这个小工具了，只需要下载对应的操作系统的版本，添加到PATH路径里，在终端窗口进入你要分享的文件所在的目录，执行可执行文件，然后将输出的地址复制下来，通过聊天工具发给在同一个局域网内的朋友，让他/她打开地址，就能从网页上点击文件名下载文件了。对的，如果你会Python开发的话，会发现和 python -m http.server 的功能是一样的，但是，我加了一点佐料。如果你需要让朋友把文件发给你，这时候你不需要让对方也安装此软件，因为页面上方可以选择文件上传。这个工具适合具备一定终端命令行能力的用户，不需要安装Python，并且能双向传输。这是个周末小练习作品，所以web上的功能很简陋，但是够用，如果你需要上传的时候使用js，有遮罩，有进度条。我会在接下来有空的周末来逐步增加实现。如果你是非GUI不可党的话，再接下来的更多周末我会想办法弄个壳出来。

## 演示视频

如果打不开Youtube 就看这里 [B站视频连接:【ru-http-sync 使用演示】](https://www.bilibili.com/video/BV1mV411Q7sR/?share_source=copy_web&vd_source=ed4c59b63c93c54db05c54ae7585495d)

[![ru-http-sync](https://res.cloudinary.com/marcomontalbano/image/upload/v1706428615/video_to_markdown/images/youtube--QZbMyvME2nA-c05b58ac6eb4c4700831b2b3070cd403.jpg)](https://youtu.be/QZbMyvME2nA "ru-http-sync")