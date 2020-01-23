const message = process.argv[2];
const emojiString1 = process.argv[3] || '-'//':partyparrot:';
const emojiString2 = process.argv[4] || '*' //':partyparrotphase2:';
​
const encodedA = [
  'AABBBBBAA',
  'ABAAAAABA',
  'ABBBBBBBA',
  'ABAAAAABA',
  'ABAAAAABA'
];
​
const encodedB = [
  'ABBBBBBAA',
  'ABAAAAABA',
  'ABBBBBBBA',
  'ABAAAAABA',
  'ABBBBBBAA',
];
​
const encodedC = [
  'ABBBBBBBA',
  'ABAAAAAAA',
  'ABAAAAAAA',
  'ABAAAAAAA',
  'ABBBBBBBA',
];
​
const encodedD = [
  'ABBBBBBAA',
  'ABAAAAABA',
  'ABAAAAABA',
  'ABAAAAABA',
  'ABBBBBBAA',
];
​
const encodedE = [
  'ABBBBBBBA',
  'ABAAAAAAA',
  'ABBBBBBBA',
  'ABAAAAAAA',
  'ABBBBBBBA',
];
​
const encodedF = [
  'ABBBBBBBA',
  'ABAAAAAAA',
  'ABBBBBBBA',
  'ABAAAAAAA',
  'ABAAAAAAA',
];
​
const encodedG = [
  'ABBBBBBBA',
  'ABAAAAAAA',
  'ABAAAABBA',
  'ABAAAAABA',
  'ABBBBBBBA',
];
​
const encodedH = [
  'ABAAAAABA',
  'ABAAAAABA',
  'ABBBBBBBA',
  'ABAAAAABA',
  'ABAAAAABA',
];
​
const encodedI = [
  'ABBBBBBBA',
  'AAAABAAAA',
  'AAAABAAAA',
  'AAAABAAAA',
  'ABBBBBBBA',
];
​
const encodedJ = [
  'ABBBBBBBA',
  'AAAAAABAA',
  'AAAAAABAA',
  'ABAAAABAA',
  'ABBBBBBAA',
];
​
const encodedK = [
  'ABAAAAABA',
  'ABAAABBAA',
  'ABBBBBAAA',
  'ABAAABBAA',
  'ABAAAAABA',
];
​
const encodedL = [
  'ABAAAAAAA',
  'ABAAAAAAA',
  'ABAAAAAAA',
  'ABAAAAAAA',
  'ABBBBBBBA',
];
​
const encodedM = [
  'ABBAAABBA',
  'ABABABABA',
  'ABAABAABA',
  'ABAABAABA',
  'ABAABAABA',
];
​
const encodedN = [
  'ABBAAAABA',
  'ABABAAABA',
  'ABAABBABA',
  'ABAAAABBA',
  'ABAAAAABA',
];
​
const encodedO = [
  'ABBBBBBBA',
  'ABAAAAABA',
  'ABAAAAABA',
  'ABAAAAABA',
  'ABBBBBBBA',
];
​
const encodedP = [
  'ABBBBBBBA',
  'ABAAAAABA',
  'ABBBBBBBA',
  'ABAAAAAAA',
  'ABAAAAAAA',
];
​
const encodedQ = [
  'ABBBBBBBA',
  'ABAAAAABA',
  'ABAAAAABA',
  'ABAAABABA',
  'ABBBBABAA',
];
​
const encodedR = [
  'ABBBBBBBA',
  'ABAAAAABA',
  'ABBBBBBBA',
  'ABAAAABAA',
  'ABAAAAABA',
];
​
const encodedS = [
  'ABBBBBBBA',
  'ABAAAAAAA',
  'AABBBBBAA',
  'AAAAAAABA',
  'ABBBBBBBA',
];
​
const encodedT = [
  'ABBBBBBBA',
  'AAAABAAAA',
  'AAAABAAAA',
  'AAAABAAAA',
  'AAAABAAAA',
];
​
const encodedU = [
  'ABAAAAABA',
  'ABAAAAABA',
  'ABAAAAABA',
  'ABAAAAABA',
  'ABBBBBBBA',
];
​
const encodedV = [
  'ABAAAAABA',
  'ABAAAAABA',
  'ABAAAAABA',
  'AABAAABAA',
  'AAABBBAAA',
];
​
const encodedW = [
  'ABAAAAABA',
  'ABAABAABA',
  'ABAABAABA',
  'ABABBBABA',
  'ABBAAABBA',
];
​
const encodedX = [
  'ABAAAAABA',
  'AABAAABAA',
  'AAABBBAAA',
  'AABAAABAA',
  'ABAAAAABA',
];
​
const encodedY = [
  'ABAAAAABA',
  'AABAAABAA',
  'AAABBBAAA',
  'AAAABAAAA',
  'AAAABAAAA',
];
​
const encodedZ = [
  'ABBBBBBBA',
  'AAAAAABAA',
  'AAAABBAAA',
  'AABAAAAAA',
  'ABBBBBBBA',
];
​
const encodedSpace = [
  'AAAAAAAAA',
  'AAAAAAAAA',
  'AAAAAAAAA',
  'AAAAAAAAA',
  'AAAAAAAAA',
];
​
const encodedExclamation = [
  'AAAABAAAA',
  'AAAABAAAA',
  'AAAABAAAA',
  'AAAAAAAAA',
  'AAAABAAAA',
];
​
const encodedQuestion = [
  'ABBBBBBBA',
  'AAAAAAABA',
  'AAAABBBA',
  'AAAAAAAAA',
  'AAAABAAAA',
];
​
const encodedPeriod = [
  'AAAAAAAAA',
  'AAAAAAAAA',
  'AAAAAAAAA',
  'AAAAAAAAA',
  'AAAABAAAA',
];
​
const encodedDash = [
  'AAAAAAAAA',
  'AAAAAAAAA',
  'ABBBBBBBA',
  'AAAAAAAAA',
  'AAAAAAAAA',
];
​
const encodedEquals = [
  'AAAAAAAAA',
  'ABBBBBBBA',
  'AAAAAAAAA',
  'ABBBBBBBA',
  'AAAAAAAAA',
];
​
const encodedPlus = [
  'AAAABAAAA',
  'AAAABAAAA',
  'ABBBBBBBA',
  'AAAABAAAA',
  'AAAABAAAA',
];
​
const encoded1 = [
  'AAAABBAAA',
  'AAABABAAA',
  'AAAAABAAA',
  'AAAAABAAA',
  'AABBBBBBA',
];
​
const encoded2 = [
  'ABBBBBBBA',
  'AAAAAAABA',
  'ABBBBBBBA',
  'ABAAAAAAA',
  'ABBBBBBBA',
];
​
const encoded3 = [
  'ABBBBBBBA',
  'AAAAAAABA',
  'AABBBBBAA',
  'AAAAAAABA',
  'ABBBBBBAA',
];
​
const encoded4 = [
  'ABAAAABAA',
  'ABAAAABAA',
  'ABBBBBBBA',
  'AAAAAABAA',
  'AAAAAABAA',
];
​
const encoded5 = [
  'ABBBBBBBA',
  'ABAAAAAAA',
  'ABBBBBBBA',
  'AAAAAAABA',
  'ABBBBBBBA',
];
​
const encoded6 = [
  'ABBBBBBBA',
  'ABAAAAAAA',
  'ABBBBBBBA',
  'ABAAAAABA',
  'ABBBBBBBA',
];
​
const encoded7 = [
  'ABBBBBBBA',
  'AAAAAABAA',
  'AAAAABAAA',
  'AAAABAAAA',
  'AAABAAAAA',
];
​
const encoded8 = [
  'ABBBBBBBA',
  'ABAAAAABA',
  'ABBBBBBBA',
  'ABAAAAABA',
  'ABBBBBBBA',
];
​
const encoded9 = [
  'ABBBBBBBA',
  'ABAAAAABA',
  'ABBBBBBBA',
  'AAAAAAABA',
  'ABBBBBBBA',
];
​
const encoded0 = [
  'ABBBBBBBA',
  'ABAAAABBA',
  'ABAABAABA',
  'ABBAAAABA',
  'ABBBBBBBA',
];
​
const encodingMap = {
  A: encodedA,
  B: encodedB,
  C: encodedC,
  D: encodedD,
  E: encodedE,
  F: encodedF,
  G: encodedG,
  H: encodedH,
  I: encodedI,
  J: encodedJ,
  K: encodedK,
  L: encodedL,
  M: encodedM,
  N: encodedN,
  O: encodedO,
  P: encodedP,
  Q: encodedQ,
  R: encodedR,
  S: encodedS,
  T: encodedT,
  U: encodedU,
  V: encodedV,
  W: encodedW,
  X: encodedX,
  Y: encodedY,
  Z: encodedZ,
  ' ': encodedSpace,
  '!': encodedExclamation,
  '?': encodedQuestion,
  '.': encodedPeriod,
  '-': encodedDash,
  '+': encodedPlus,
  '=': encodedEquals,
  '1': encoded1,
  '2': encoded2,
  '3': encoded3,
  '4': encoded4,
  '5': encoded5,
  '6': encoded6,
  '7': encoded7,
  '8': encoded8,
  '9': encoded9,
  '0': encoded0,
};
​
const charHeight = 5;
​
const messageLines = message.split('\n');
​
for (var x = 0; x < messageLines.length; x++) {
  const sublines = generateLine(messageLines[x]);
  const isLastLine = x === messageLines.length - 1;
  printLine(sublines, isLastLine);
}
​
function generateLine(message) {
  const sublines = [];
​
  for (var x = 0; x < charHeight; x++) {
    sublines.push('')
  };
​
  for(var x = 0; x < message.length; x ++) {
    const encoding = encodingMap[message[x].toUpperCase()];
    if (encoding) {
      for (var y = 0; y < charHeight; y ++) {
        sublines[y] += encoding[y];
      }
    }
  }
  return sublines;
}
​
function printLine(sublines, isLastLine) {
  for (var x = 0; x < charHeight; x++) {
    sublines[x] = sublines[x].replace(/A/g, emojiString1);
    sublines[x] = sublines[x].replace(/B/g, emojiString2);
    console.log(sublines[x])
  };
  if (!isLastLine) {
    console.log('')
  }
}
​