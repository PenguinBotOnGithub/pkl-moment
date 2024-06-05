import React from "react";
import { useParams } from "react-router-dom";

function Entry() {
  let { id, entry } = useParams();
  return <div>Entry {id} {entry} </div>;
}

export default Entry;
