CREATE TABLE Afstemning (
        id INTEGER NOT NULL,
        nummer INTEGER NOT NULL,
        konklusion TEXT NULL,
        vedtaget BOOLEAN NOT NULL,
        kommentar TEXT NULL,
        mødeid INTEGER NOT NULL,
        typeid INTEGER NOT NULL,
        sagstrinid INTEGER NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Afstemningstype (
        id INTEGER NOT NULL,
        type TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Aktstykke (
        id INTEGER NOT NULL,
        typeid INTEGER NOT NULL,
        kategoriid INTEGER NULL,
        statusid INTEGER NOT NULL,
        titel TEXT NULL,
        titelkort TEXT NULL,
        offentlighedskode TEXT NULL,
        nummer TEXT NULL,
        nummerprefix TEXT NULL,
        nummernumerisk TEXT NULL,
        nummerpostfix TEXT NULL,
        resume TEXT NULL,
        afstemningskonklusion TEXT NULL,
        periodeid INTEGER NOT NULL,
        afgørelsesresultatkode TEXT NULL,
        baggrundsmateriale TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        statsbudgetsag BOOLEAN NULL,
        begrundelse TEXT NULL,
        paragrafnummer INTEGER NULL,
        paragraf TEXT NULL,
        afgørelsesdato DATETIME(3) NULL,
        afgørelse TEXT NULL,
        rådsmødedato DATETIME(3) NULL,
        lovnummer TEXT NULL,
        lovnummerdato DATETIME(3) NULL,
        retsinformationsurl TEXT NULL,
        fremsatundersagid INTEGER NULL,
        deltundersagid INTEGER NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Aktør (
        id INTEGER NOT NULL,
        typeid INTEGER NOT NULL,
        gruppenavnkort TEXT NULL,
        navn TEXT NULL,
        fornavn TEXT NULL,
        efternavn TEXT NULL,
        biografi TEXT NULL,
        periodeid INTEGER NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        startdato DATETIME(3) NULL,
        slutdato DATETIME(3) NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE AktørAktør (
        id INTEGER NOT NULL,
        fraaktørid INTEGER NOT NULL,
        tilaktørid INTEGER NOT NULL,
        startdato DATETIME(3) NULL,
        slutdato DATETIME(3) NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        rolleid INTEGER NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE AktørAktørRolle (
        id INTEGER NOT NULL,
        rolle TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Aktørtype (
        id INTEGER NOT NULL,
        type TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Almdel (
        id INTEGER NOT NULL,
        typeid INTEGER NOT NULL,
        kategoriid INTEGER NULL,
        statusid INTEGER NOT NULL,
        titel TEXT NULL,
        titelkort TEXT NULL,
        offentlighedskode TEXT NULL,
        nummer TEXT NULL,
        nummerprefix TEXT NULL,
        nummernumerisk TEXT NULL,
        nummerpostfix TEXT NULL,
        resume TEXT NULL,
        afstemningskonklusion TEXT NULL,
        periodeid INTEGER NOT NULL,
        afgørelsesresultatkode TEXT NULL,
        baggrundsmateriale TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        statsbudgetsag BOOLEAN NULL,
        begrundelse TEXT NULL,
        paragrafnummer INTEGER NULL,
        paragraf TEXT NULL,
        afgørelsesdato DATETIME(3) NULL,
        afgørelse TEXT NULL,
        rådsmødedato DATETIME(3) NULL,
        lovnummer TEXT NULL,
        lovnummerdato DATETIME(3) NULL,
        retsinformationsurl TEXT NULL,
        fremsatundersagid INTEGER NULL,
        deltundersagid INTEGER NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Dagsordenspunkt (
        id INTEGER NOT NULL,
        kørebemærkning TEXT NULL,
        titel TEXT NULL,
        kommentar TEXT NULL,
        nummer TEXT NULL,
        forhandlingskode TEXT NULL,
        forhandling TEXT NULL,
        superid INTEGER NULL,
        sagstrinid INTEGER NULL,
        mødeid INTEGER NOT NULL,
        offentlighedskode TEXT NULL,
        opdateringsdato DATETIME(3) NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE DagsordenspunktDokument (
        id INTEGER NOT NULL,
        dokumentid INTEGER NOT NULL,
        dagsordenspunktid INTEGER NOT NULL,
        note TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE DagsordenspunktSag (
        id INTEGER NOT NULL,
        dagsordenspunktid INTEGER NOT NULL,
        sagid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Debat (
        id INTEGER NOT NULL,
        typeid INTEGER NOT NULL,
        kategoriid INTEGER NULL,
        statusid INTEGER NOT NULL,
        titel TEXT NULL,
        titelkort TEXT NULL,
        offentlighedskode TEXT NULL,
        nummer TEXT NULL,
        nummerprefix TEXT NULL,
        nummernumerisk TEXT NULL,
        nummerpostfix TEXT NULL,
        resume TEXT NULL,
        afstemningskonklusion TEXT NULL,
        periodeid INTEGER NOT NULL,
        afgørelsesresultatkode TEXT NULL,
        baggrundsmateriale TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        statsbudgetsag BOOLEAN NULL,
        begrundelse TEXT NULL,
        paragrafnummer INTEGER NULL,
        paragraf TEXT NULL,
        afgørelsesdato DATETIME(3) NULL,
        afgørelse TEXT NULL,
        rådsmødedato DATETIME(3) NULL,
        lovnummer TEXT NULL,
        lovnummerdato DATETIME(3) NULL,
        retsinformationsurl TEXT NULL,
        fremsatundersagid INTEGER NULL,
        deltundersagid INTEGER NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Dokument (
        id INTEGER NOT NULL,
        typeid INTEGER NOT NULL,
        kategoriid INTEGER NOT NULL,
        statusid INTEGER NOT NULL,
        offentlighedskode TEXT NULL,
        titel TEXT NULL,
        dato DATETIME(3) NOT NULL,
        modtagelsesdato DATETIME(3) NULL,
        frigivelsesdato DATETIME(3) NULL,
        paragraf TEXT NULL,
        paragrafnummer TEXT NULL,
        spørgsmålsordlyd TEXT NULL,
        spørgsmålstitel TEXT NULL,
        spørgsmålsid INTEGER NULL,
        procedurenummer TEXT NULL,
        grundnotatstatus TEXT NULL,
        dagsordenudgavenummer INTEGER NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE DokumentAktør (
        id INTEGER NOT NULL,
        dokumentid INTEGER NOT NULL,
        aktørid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        rolleid INTEGER NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE DokumentAktørRolle (
        id INTEGER NOT NULL,
        rolle TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Dokumentkategori (
        id INTEGER NOT NULL,
        kategori TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Dokumenttype (
        id INTEGER NOT NULL,
        type TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Dokumentstatus (
        id INTEGER NOT NULL,
        status TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Emneord (
        id INTEGER NOT NULL,
        typeid INTEGER NOT NULL,
        emneord TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE EmneordDokument (
        id INTEGER NOT NULL,
        emneordid INTEGER NOT NULL,
        dokumentid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE EmneordSag (
        id INTEGER NOT NULL,
        emneordid INTEGER NOT NULL,
        sagid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Emneordstype (
        id INTEGER NOT NULL,
        type TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE EUsag (
        id INTEGER NOT NULL,
        typeid INTEGER NOT NULL,
        kategoriid INTEGER NULL,
        statusid INTEGER NOT NULL,
        titel TEXT NULL,
        titelkort TEXT NULL,
        offentlighedskode TEXT NULL,
        nummer TEXT NULL,
        nummerprefix TEXT NULL,
        nummernumerisk TEXT NULL,
        nummerpostfix TEXT NULL,
        resume TEXT NULL,
        afstemningskonklusion TEXT NULL,
        periodeid INTEGER NOT NULL,
        afgørelsesresultatkode TEXT NULL,
        baggrundsmateriale TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        statsbudgetsag BOOLEAN NULL,
        begrundelse TEXT NULL,
        paragrafnummer INTEGER NULL,
        paragraf TEXT NULL,
        afgørelsesdato DATETIME(3) NULL,
        afgørelse TEXT NULL,
        rådsmødedato DATETIME(3) NULL,
        lovnummer TEXT NULL,
        lovnummerdato DATETIME(3) NULL,
        retsinformationsurl TEXT NULL,
        fremsatundersagid INTEGER NULL,
        deltundersagid INTEGER NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Forslag (
        id INTEGER NOT NULL,
        typeid INTEGER NOT NULL,
        kategoriid INTEGER NULL,
        statusid INTEGER NOT NULL,
        titel TEXT NULL,
        titelkort TEXT NULL,
        offentlighedskode TEXT NULL,
        nummer TEXT NULL,
        nummerprefix TEXT NULL,
        nummernumerisk TEXT NULL,
        nummerpostfix TEXT NULL,
        resume TEXT NULL,
        afstemningskonklusion TEXT NULL,
        periodeid INTEGER NOT NULL,
        afgørelsesresultatkode TEXT NULL,
        baggrundsmateriale TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        statsbudgetsag BOOLEAN NULL,
        begrundelse TEXT NULL,
        paragrafnummer INTEGER NULL,
        paragraf TEXT NULL,
        afgørelsesdato DATETIME(3) NULL,
        afgørelse TEXT NULL,
        rådsmødedato DATETIME(3) NULL,
        lovnummer TEXT NULL,
        lovnummerdato DATETIME(3) NULL,
        retsinformationsurl TEXT NULL,
        fremsatundersagid INTEGER NULL,
        deltundersagid INTEGER NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Fil (
        id INTEGER NOT NULL,
        dokumentid INTEGER NOT NULL,
        titel TEXT NULL,
        versionsdato DATETIME(3) NOT NULL,
        filurl TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        variantkode TEXT NULL,
        format TEXT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE KolloneBeskrivelse (
        id INTEGER NOT NULL,
        entitetnavn TEXT NULL,
        kollonenavn TEXT NULL,
        beskrivelse TEXT NULL,
        opdateringsdato DATETIME(3) NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE EntitetBeskrivelse (
        id INTEGER NOT NULL,
        entitetnavn TEXT NULL,
        beskrivelse TEXT NULL,
        opdateringsdato DATETIME(3) NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Møde (
        id INTEGER NOT NULL,
        titel TEXT NULL,
        lokale TEXT NULL,
        nummer TEXT NULL,
        dagsordenurl TEXT NULL,
        starttidsbemærkning TEXT NULL,
        offentlighedskode TEXT NULL,
        dato DATETIME(3) NULL,
        statusid INTEGER NULL,
        typeid INTEGER NULL,
        periodeid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE MødeAktør (
        id INTEGER NOT NULL,
        mødeid INTEGER NOT NULL,
        aktørid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Mødestatus (
        id INTEGER NOT NULL,
        status TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Mødetype (
        id INTEGER NOT NULL,
        type TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Omtryk (
        id INTEGER NOT NULL,
        dokumentid INTEGER NOT NULL,
        dato DATETIME(3) NULL,
        begrundelse TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Periode (
        id INTEGER NOT NULL,
        startdato DATETIME(3) NOT NULL,
        slutdato DATETIME(3) NOT NULL,
        type TEXT NULL,
        kode TEXT NULL,
        titel TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Sag (
        id INTEGER NOT NULL,
        typeid INTEGER NOT NULL,
        kategoriid INTEGER NULL,
        statusid INTEGER NOT NULL,
        titel TEXT NULL,
        titelkort TEXT NULL,
        offentlighedskode TEXT NULL,
        nummer TEXT NULL,
        nummerprefix TEXT NULL,
        nummernumerisk TEXT NULL,
        nummerpostfix TEXT NULL,
        resume TEXT NULL,
        afstemningskonklusion TEXT NULL,
        periodeid INTEGER NOT NULL,
        afgørelsesresultatkode TEXT NULL,
        baggrundsmateriale TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        statsbudgetsag BOOLEAN NULL,
        begrundelse TEXT NULL,
        paragrafnummer INTEGER NULL,
        paragraf TEXT NULL,
        afgørelsesdato DATETIME(3) NULL,
        afgørelse TEXT NULL,
        rådsmødedato DATETIME(3) NULL,
        lovnummer TEXT NULL,
        lovnummerdato DATETIME(3) NULL,
        retsinformationsurl TEXT NULL,
        fremsatundersagid INTEGER NULL,
        deltundersagid INTEGER NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE SagAktør (
        id INTEGER NOT NULL,
        aktørid INTEGER NOT NULL,
        sagid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        rolleid INTEGER NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE SagAktørRolle (
        id INTEGER NOT NULL,
        rolle TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE SagDokument (
        id INTEGER NOT NULL,
        sagid INTEGER NOT NULL,
        dokumentid INTEGER NOT NULL,
        bilagsnummer TEXT NULL,
        frigivelsesdato DATETIME(3) NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        rolleid INTEGER NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE SagDokumentRolle (
        id INTEGER NOT NULL,
        rolle TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Sagskategori (
        id INTEGER NOT NULL,
        kategori TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Sagsstatus (
        id INTEGER NOT NULL,
        status TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Sagstrin (
        id INTEGER NOT NULL,
        titel TEXT NULL,
        dato DATETIME(3) NULL,
        sagid INTEGER NOT NULL,
        typeid INTEGER NOT NULL,
        folketingstidendeurl TEXT NULL,
        folketingstidende TEXT NULL,
        folketingstidendesidenummer TEXT NULL,
        statusid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE SagstrinAktør (
        id INTEGER NOT NULL,
        sagstrinid INTEGER NOT NULL,
        aktørid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        rolleid INTEGER NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE SagstrinAktørRolle (
        id INTEGER NOT NULL,
        rolle TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Sambehandlinger (
        id INTEGER NOT NULL,
        førstesagstrinid INTEGER NOT NULL,
        andetsagstrinid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE SagstrinDokument (
        id INTEGER NOT NULL,
        sagstrinid INTEGER NOT NULL,
        dokumentid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Sagstrinsstatus (
        id INTEGER NOT NULL,
        status TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Sagstrinstype (
        id INTEGER NOT NULL,
        type TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Sagstype (
        id INTEGER NOT NULL,
        type TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Stemme (
        id INTEGER NOT NULL,
        typeid INTEGER NULL,
        afstemningid INTEGER NOT NULL,
        aktørid INTEGER NOT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);

CREATE TABLE Stemmetype (
        id INTEGER NOT NULL,
        type TEXT NULL,
        opdateringsdato DATETIME(3) NOT NULL,
        PRIMARY KEY (id, opdateringsdato)
);