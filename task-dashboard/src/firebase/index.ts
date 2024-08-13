import { initializeApp } from "firebase/app";
import { collection, getFirestore } from "firebase/firestore";

const firebaseConfig = {
  apiKey: "AIzaSyBPbDBwpKVQLR_uy_Z7UEK1hdQqmXE-rZ4",

  authDomain: "study-381803.firebaseapp.com",

  projectId: "study-381803",

  storageBucket: "study-381803.appspot.com",

  messagingSenderId: "825409923820",

  appId: "1:825409923820:web:23a4a859642f9effdd13c9",
};

const app = initializeApp(firebaseConfig);

export const db = getFirestore(app);
export const sysinfoCollection = collection(db, "system_info");
