// 引入 fs 和 path 模块
const fs = require('fs');
const path = require('path');

// 正则匹配获取源文件信息
const getInfoFromFile = (rustCode) => {
  // 定义一个正则表达式，用来匹配函数名、函数列表、函数返回值信息
  const regex = /fn\s+(\w+)\s*\(([^)]*)\)\s*(->\s*(\w+))?/;

  // 使用正则表达式的 exec 方法来执行匹配，并且获取匹配结果
  const result = regex.exec(rustCode);
  const infos = {};

  // 如果匹配成功，打印匹配结果
  if (result) {
    // result[0] 是整个匹配的字符串
    infos.source = result[0];

    // result[1] 是第一个捕获组，即函数名
    infos.functionName = result[1];

    // result[2] 是第二个捕获组，即函数入参列表
    infos.argsWithType = result[2]?.trim();
    if (infos.argsWithType) {
      const argList = infos.argsWithType.split(',');
      infos.args = argList.map((item) => {
        const nameAndType = item.split(':');
        return {
          name: nameAndType[0]?.trim() || '',
          type: nameAndType[1]?.trim() || '',
        };
      });
    }

    // result[3] 是第三个捕获组，即函数返回值信息（有可能没有）
    infos.returnTypeStr = result[3];
    infos.returnType = result[4];
  }

  return infos;
};

const getFileName = (fileName) => {
  // 获取文件的扩展名
  const ext = path.extname(fileName);
  // 返回不带扩展名的文件名
  return path.basename(fileName, ext);
}

// 获取目标目录下文件名列表（排除文件夹情况、不需要扩展名）
const getFileNameList = (dirPath) => {
  const fileList = fs.readdirSync(dirPath);
  return fileList
    .filter((item) => {
      const stat = fs.statSync(path.join(dirPath, item));
      return stat.isFile();
    })
    .map((file) => {
      return getFileName(file);
    });
};

module.exports = {
  getInfoFromFile,
  getFileNameList,
  getFileName,
};
