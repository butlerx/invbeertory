import { purchasesBy, stockBy } from './occurences';

export * from './sort';
export * from './colours';
export * from './dates';
export * from './filters';
export const purchasesByStyle = purchasesBy('style');
export const purchasesByBrewery = purchasesBy('brewery');
export const stockByAbv = stockBy('abv');

export const inObject = (key: string, obj: {}): boolean =>
  key in obj && obj[key] !== undefined && typeof obj[key] !== 'undefined' && obj[key] !== null;
