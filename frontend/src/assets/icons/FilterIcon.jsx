import React from 'react';

export const FilterIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <path stroke={stroke} stroke-linejoin="round" stroke-width="12" d="M112.498 95.99v64.782c.33 2.467-.495 5.097-2.392 6.824A8.22 8.22 0 0 1 104.29 170a8.233 8.233 0 0 1-5.815-2.404l-16.58-16.525a8.12 8.12 0 0 1-2.393-6.823V95.99L79.5 96 31.741 35.318a8.201 8.201 0 0 1 1.403-11.51c1.567-1.15 3.3-1.808 5.114-1.808h115.484c1.815 0 3.547.658 5.114 1.809a8.202 8.202 0 0 1 1.403 11.51L112.5 96l-.002-.01Z"/>
</svg>

  );
};