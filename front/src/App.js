import React from "react";
import "./App.css";
import Login from "./components/Login";
import Home from "./components/Home";
import JarRanking from "./components/JarRanking";
import JarSearch from "./components/JarSearch";

import { BrowserRouter as Router, Route, Switch } from "react-router-dom";

function App() {
  return (
    <>
      <Router>
        <Switch>
          <Route path="/" exact component={Login} />
          <Route path="/jars" exact component={Home} />
          <Route path="/jars/ranking" exact component={JarRanking} />
          <Route path="/jars/search" exact component={JarSearch} />
        </Switch>
      </Router>
    </>
  );
}

export default App;
