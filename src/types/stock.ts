import { Beer } from './beer';

export interface StockProps {
  data: {
    allGoogleInventorySheet: {
      nodes: Beer[];
    };
  };
}
