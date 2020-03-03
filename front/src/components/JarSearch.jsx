import React, { useState, useEffect } from "react";
import { Link } from "react-router-dom";

import "../css/Home.css";
import untoggled from "../images/check_untoggled.svg";
import toggled from "../images/check_toggled.svg";
import checkmark from "../images/close.svg";

const JarSearch = () => {
  let [jars, setJars] = useState([]);
  let [searchValue, setSearchValue] = useState("");
  let [relevantJars, setRelevantJars] = useState([]);
  let [checked, setChecked] = useState([]);

  useEffect(() => {
    let jar_list = [];
    let query = fetch("/jars");
    query
      .then(data => data.json())
      .then(json => {
        json.forEach(js => {
          jar_list.push(JSON.parse(js));
        });
      })
      .then(() => {
        setJars(jar_list);
      });
  }, []);

  useEffect(() => {
    let filtered = jars.filter(jar => {
      console.log(searchValue);
      console.log(jar.nickname);
      return jar.nickname.toLowerCase().startsWith(searchValue);
    });
    setRelevantJars(filtered);
  }, [searchValue, jars]);

  let handleCheckedRemove = id => {
    let copy = [...checked];
    copy = copy.filter(jar => jar.id !== id);
    setChecked(copy);
  };

  let handleCheckedNew = jar => {
    if (checked.find(jr => jr.id === jar.id)) {
      return;
    }
    let copy = [...checked];
    copy.push(jar);
    setChecked(copy);
    updateQuery(jar);
  };

  let updateQuery = jar => {
    let query = fetch(`/increase-counter/${jar.id}`);
    query.then(data => {
      console.log(data);
    });
  };

  let renderChecked = () => {
    return checked.map(jar => {
      return (
        <div className="row">
          <div className="col-9 check-col">
            <strong> {`${jar.nickname}: ${jar.place}`} </strong>
          </div>
          <div className="col check-col">
            <button
              className="btn btn-check"
              style={{ background: "none" }}
              onClick={() => handleCheckedRemove(jar.id)}
            >
              <img src={checkmark} className="checked-img smaller"></img>
            </button>
          </div>
        </div>
      );
    });
  };

  let unOrtog = id => {
    return checked.find(jr => jr.id === id) ? toggled : untoggled;
  };

  let renderItems = () => {
    return searchValue !== "" ? (
      relevantJars.map(jar => {
        return (
          <div className="jar-item" key={jar.id}>
            <div className="row">
              <div className="col-10">
                <h2> {jar.nickname} </h2>
              </div>
              <div className="col-2 check-col">
                <button
                  onClick={() => handleCheckedNew(jar)}
                  className="btn btn-check"
                  style={{ background: "none" }}
                >
                  <img src={unOrtog()} className={"checked-img"}></img>
                </button>
              </div>
            </div>
            <h4>{jar.place}</h4>
            <small>{jar.bar}</small>
          </div>
        );
      })
    ) : (
      <></>
    );
  };

  return (
    <div className="container center-div">
      <div className="irish-title">
        <h1>Buscar</h1>
      </div>
      <input
        className="jar-search-bar form-control"
        type="text"
        placeholder="Busca un jarro!"
        onChange={e => setSearchValue(e.target.value.toLowerCase())}
      />
      <div className="checked-list">{renderChecked()}</div>
      <div className="jar-list">{renderItems()}</div>
      <div className="row btn-row">
        <Link className="btn irish-btn bottom-btn" to={`/jars`}>
          Back
        </Link>
      </div>
    </div>
  );
};

export default JarSearch;
