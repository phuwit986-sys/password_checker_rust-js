const readline = require('readline');
const MIN_LEN = 15;

const askUsername = () => new Promise((res) => {
  const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
  rl.question('Username (email): ', (a) => { res(a.trim()); rl.close(); });
});

const askPassword = () => new Promise((res) => {
  const chars = [];  // เก็บทีละตัวอักษร ไม่ trim ไม่ตัด
  const stdin = process.stdin;
  const onData = (chunk) => {
    if (chunk[0] === '\x1b') return;  // ข้ามปุ่มลูกศร
    for (const ch of chunk) {
      if (ch === '\r' || ch === '\n') {
        stdin.setRawMode(false); stdin.pause(); stdin.off('data', onData);
        console.log();
        return res(chars.join(''));
      }
      if (ch === '\x03') { console.log(); process.exit(130); }
      if (ch === '\x7f' || ch === '\b') { if (chars.pop() !== undefined) process.stdout.write('\b \b'); continue; }
      if (ch >= ' ') { chars.push(ch); process.stdout.write('*'); }
    }
  };
  process.stdout.write('Password: ');
  stdin.setRawMode(true); stdin.setEncoding('utf8'); stdin.resume(); stdin.on('data', onData);
});

(async () => {
  for (let i = 1; i <= 3; i++) {
    const user = await askUsername();
    const pass = await askPassword();
    const len = [...pass].length;  // นับเป็นตัวอักษร (ไม่ใช่ pass.length ที่นับ emoji เป็น 2)
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(user)) console.log(`ไม่ผ่าน (${i}/3): username ต้องเป็นอีเมล\n`);
    else if (len < MIN_LEN) console.log(`ไม่ผ่าน (${i}/3): รหัสผ่านสั้นเกินไป (${len}/${MIN_LEN} ตัวอักษร)\n`);
    else return console.log(`ผ่าน: ความยาว ${len} ตัวอักษร\nWelcome ${user}`);
  }
  console.log('ผิดครบ 3 ครั้ง Logout!');
})();
