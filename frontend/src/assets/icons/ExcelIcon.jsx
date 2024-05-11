import React from 'react';

export const ExcelIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <path stroke={stroke} stroke-linejoin="round" stroke-width="12" d="M22 45.569a6 6 0 0 1 5.054-5.925l66-10.535C96.7 28.527 100 31.343 100 35.034v121.932c0 3.691-3.3 6.507-6.946 5.925l-66-10.535A6 6 0 0 1 22 146.431V45.569Z"/>
  <rect width={size} height={size} x="100" y="38" stroke={stroke} stroke-linejoin="round" stroke-width="12" rx="6"/>
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="m45 74 28 48m0-52-28 48"/>
</svg>

  );
};