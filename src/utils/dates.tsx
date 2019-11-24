const months = [
  'January',
  'February',
  'March',
  'April',
  'May',
  'June',
  'July',
  'August',
  'September',
  'October',
  'November',
  'December',
];

export const getMonth = (i: number): string => months[i];

export const formatDate = (date?: Date): string =>
  date === undefined || date === null
    ? 'present'
    : `${getMonth(new Date(date).getMonth())} ${new Date(date).getDay()}, ${new Date(
        date,
      ).getFullYear()}`;
