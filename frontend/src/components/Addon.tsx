import React from 'react';
import logo from './logo.svg';

export type AddonProps = {
  name: string;
}

export const Addon = (props: AddonProps) => {
  return (
    <div className="Addon">
      {props.name}
    </div>
  )
};
