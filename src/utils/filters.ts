export function filterCaseInsensitive(filter, row) {
  const content = row[filter.pivotId || filter.id];
  if (typeof content !== 'undefined') {
    // filter by text in the table or if it's a object, filter by key
    return typeof content === 'object' && content !== null && content.key
      ? String(content.key)
          .toLowerCase()
          .includes(filter.value.toLowerCase())
      : String(content)
          .toLowerCase()
          .includes(filter.value.toLowerCase());
  }
  return true;
}
