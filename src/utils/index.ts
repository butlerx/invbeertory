import { purchasesBy, stockBy } from './occurences';

export * from './colours';
export * from './dates';
export * from './filters';
export * from './object';
export * from './style';

export const purchasesByStyle = purchasesBy('style');
export const purchasesByBrewery = purchasesBy('brewery');
export const stockByAbv = stockBy('abv');
