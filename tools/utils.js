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

module.exports = {
  getInfoFromFile,
};