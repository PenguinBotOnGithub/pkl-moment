import React from 'react';

export const DashboardIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <rect width={size} height={size} x="22" y="22" stroke={stroke} stroke-linejoin="round" stroke-width="12" rx="6"/>
  <rect width={size} height={size} x="22" y="107" stroke={stroke} stroke-linejoin="round" stroke-width="12" rx="6"/>
  <rect width={size} height={size} x="107" y="22" stroke={stroke} stroke-linejoin="round" stroke-width="12" rx="6"/>
  <rect width={size} height={size} x="107" y="107" stroke={stroke} stroke-linejoin="round" stroke-width="12" rx="6"/>
</svg>

  );
};