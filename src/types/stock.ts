import { Beer } from './beer';
import { Coffee } from './coffee';

export interface StockProps {
  data: {
    allGoogleSheetInventoryRow: {
      nodes: (Beer | Coffee)[];
    };
  };
}
