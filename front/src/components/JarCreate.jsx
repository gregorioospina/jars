import React from "react";
import { Link } from "react-router-dom";
import "../css/Home.css";

const JarCreate = props => {
  const options = ["Usaquén", "Quinta Camacho", "Candelaria", "La T"];

  return (
    <div className="container">
      <div className="center-div irish-title">
        <h1>Crear</h1>
      </div>
      <div className="create-jar-form">
        <div className="form-group">
          <input
            type="text"
            className="jar-search-bar form-control"
            placeholder="Nombre"
          />
        </div>
        <div className="form-group">
          <input
            type="text"
            className="jar-search-bar form-control"
            placeholder="Donde lo van a Guardar?"
          />
        </div>
        <div className="form-group">
          <select
            placeholder="Nombre"
            className="form-control"
            id="barOptionsSelect"
          >
            <option>Usaquén</option>
            <option>Quinta Camacho</option>
            <option>Candelaria</option>
            <option>La T</option>
          </select>
        </div>
        <div className="create-button-div center-div">
          <button type="button" className="btn irish-btn create">
            Crear
          </button>
        </div>
        <div className="row btn-row center-div">
          <Link className="btn irish-btn bottom-btn" to={`/jars`}>
            Back
          </Link>
        </div>
      </div>
    </div>
  );
};

export default JarCreate;
