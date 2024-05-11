import React from 'react';

export const WarningIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="M96 74v41"/>
  <circle cx="96" cy="137" r="4" stroke={stroke} stroke-width="8"/>
  <path stroke={stroke} stroke-linejoin="round" stroke-width="12" d="M93.395 32.558c1.152-2.015 4.058-2.015 5.21 0l70.83 123.954c1.143 2-.301 4.488-2.605 4.488H25.17c-2.304 0-3.748-2.488-2.605-4.488l70.83-123.954Z"/>
</svg>

  );
};