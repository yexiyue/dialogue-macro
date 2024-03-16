//设置机器翻译服务通道，直接客户端本身，不依赖服务端 。相关说明参考 http://translate.zvo.cn/43086.html
// translate.service.use("client.edge");
translate.ignore.class.push("icon-button");
translate.ignore.class.push("theme-popup");
//进行翻译
translate.execute();
translate.request.listener.start();