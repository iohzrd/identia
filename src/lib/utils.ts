export function getLongestString(a: string[]) {
  return a.reduce(
    (savedText, text) =>
      typeof text == "string" && text.length > savedText.length
        ? text
        : savedText,
    ""
  );
}
