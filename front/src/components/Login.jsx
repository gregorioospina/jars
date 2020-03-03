import React from "react";
import { Link } from "react-router-dom";
import "../css/Home.css";

const Login = () => {
  return (
    <div className="container center-div">
      <div className="title-login">
        <h1> Irish </h1>
      </div>
      <form>
        <div className="form-group">
          <input
            className=" jar-search-bar form-control"
            type="text"
            placeholder="Código"
          />
        </div>
        <div className="form-group">
          <button type="submit" className="btn login-btn">
            <Link className="nav-link" to={"/jars"}>
              Log In
            </Link>
          </button>
        </div>
      </form>
    </div>
  );
};

export default Login;
