// 引入 fs 和 path 模块
const fs = require('fs');
const path = require('path');
const { getFileNameList, getFileName } = require('../tools/utils');

// 引入 handlebars 模块
const handlebars = require('handlebars');

// 定义 src 目录的路径
const srcPath = path.join(__dirname, '../src');
const toolsPath = path.join(__dirname, '../tools');

const skipFileNames = ['main', 'tests'];

// 获取有意义的文件名列表
const fileNames = getFileNameList(srcPath).filter(
  (fileName) => !skipFileNames.includes(fileName)
);

console.log(fileNames);

// 定义模板文件的路径
const templatePaths = [path.join(toolsPath, 'files/main.hbs'), path.join(toolsPath, 'files/tests.hbs')];

templatePaths.forEach(templatePath =>{
  // 同步读取模板文件内容
  const templateContent = fs.readFileSync(templatePath, 'utf8');
  // 编译模板文件，得到一个渲染函数
  const render = handlebars.compile(templateContent);

  // 使用渲染函数，传入数据对象，得到渲染后的 HTML 字符串
  const resultText = render({ files: fileNames });

  // 获取想要生成的文件名
  const targetFileName = getFileName(templatePath);
  // 获取文件路径
  const targetPath = path.join(srcPath, `${targetFileName}.rs`);

  // 打印渲染后的字符串
  fs.writeFile(targetPath, resultText, (err) => {
    if (err) {
      console.error(err);
      return;
    }
  });
});
