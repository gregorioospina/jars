import React, { useState, useEffect } from "react";
import "../css/Home.css";
import { Link } from "react-router-dom";

const JarRanking = props => {
  let [jars, setJars] = useState([]);

  useEffect(() => {
    let list = [];
    const fetchJars = fetch("/jars");
    fetchJars
      .then(data => {
        return data.json();
      })
      .then(json => {
        json.forEach(jr => {
          list.push(JSON.parse(jr));
        });
      })
      .then(() => {
        list.sort((a, b) => a.queries < b.queries);
        setJars(list);
      });
  }, []);

  let render_jars = () => {
    return jars.map((jr, index) => {
      return (
        <div className="row">
          <div className="col-10">
            <h2 className={`ranking-jar ranking-jar-${index}`}>
              {jr.nickname}
            </h2>
          </div>
          <div className="col-2">
            <h2> {`${jr.queries}`}</h2>
          </div>
        </div>
      );
    });
  };
  return (
    <div className="container center-div">
      <div className="irish-title">
        <h1>Ranking</h1>
        <strong>Los jarros que mas veces han venido en el ultimo mes</strong>
      </div>
      <div className="container scroll-parent">
        <div className="jars-render">{render_jars()}</div>
      </div>
      <div className="row btn-row">
        <Link className="btn irish-btn bottom-btn" to={`/jars`}>
          Back
        </Link>
      </div>
    </div>
  );
};

export default JarRanking;
