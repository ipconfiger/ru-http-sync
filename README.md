# ru-http-sync
A tool can easily share files on LAN

[中文文档](./README_ZH.md)

## Description

If you need to share large files with friends over a local area network (LAN), but don’t want to relay it through public network chat tools (which can be slow), you can use this small tool. You only need to download the version corresponding to your operating system, add it to your PATH, navigate to the directory of the file you want to share in the terminal window, and execute the file. Then, copy the output address and send it to your friend on the same LAN via chat tools. Your friend can open the address and click on the file name on the webpage to download the file. Yes, if you are familiar with Python development, you will find that the functionality is the same as python -m http.server. However, I’ve added a little extra. If you need your friend to send a file to you, you don’t need to have them install this software, because they can choose to upload files from the webpage. This tool is suitable for users with some command line skills, does not require Python installation, and supports bidirectional transfer. This is a weekend exercise project, so the web functionality is very basic, but sufficient. If you need to use JavaScript when uploading, there is a mask and a progress bar. I will gradually add more features on subsequent weekends when I have time. If you are a GUI-only user, I will try to create a shell in the following weekends.

