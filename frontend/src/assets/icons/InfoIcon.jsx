import React from 'react';

export const InfoIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <circle cx="96" cy="96" r="74" stroke={stroke} stroke-width="12"/>
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="M96 88v41"/>
  <circle cx="96" cy="66" r="4" stroke={stroke} stroke-width="8"/>
</svg>

  );
};