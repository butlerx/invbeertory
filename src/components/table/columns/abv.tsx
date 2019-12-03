import React from 'react';
import { Beer } from '../../../types';
import { right } from '../style.module.scss';

export const abv = ({ row }: { row: Beer }) => <div className={right}>{row.abv}%</div>;
