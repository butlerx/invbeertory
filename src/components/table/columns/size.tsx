import React from 'react';
import { Beer } from '../../../types';
import { center } from '../style.module.scss';

export const size = ({ row }: { row: Beer }) => <div className={center}>{row.size} ml</div>;
