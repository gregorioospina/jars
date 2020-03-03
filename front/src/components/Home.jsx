import React from "react";
import { Link } from "react-router-dom";

import beer from "../images/beer.svg";
import medal from "../images/medal.svg";

const Home = () => {
  return (
    <div className="container center-div">
      <div className="irish-title">
        <h1> Irish </h1>
      </div>
      <div className="card menu-btn" href="jars/search">
        <img className="card-img-top svg-img" src={beer} alt="jar search" />
        <div className="card-body">
          <Link className="menu-link" to={`jars/search`}>
            <h2> Busca un Jarro </h2>
          </Link>
        </div>
      </div>
      <br />
      <div className="card menu-btn">
        <img className="card-img-top svg-img" src={medal} alt="jar search" />
        <div className="card-body">
          <Link className="menu-link" to={`jars/ranking`}>
            <h2> Ranking de Jarros </h2>
          </Link>
        </div>
      </div>
      <div className="row btn-row">
        <Link className="btn irish-btn bottom-btn" to={`/`}>
          Back
        </Link>
      </div>
    </div>
  );
};

export default Home;
