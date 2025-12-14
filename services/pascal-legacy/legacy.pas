program LegacyCSV;

{$mode objfpc}{$H+}

uses
  SysUtils, DateUtils, Process, Classes, Math;

function GetEnvDef(const name, def: string): string;
var v: string;
begin
  v := GetEnvironmentVariable(name);
  if v = '' then Exit(def) else Exit(v);
end;

function RandFloat(minV, maxV: Double): Double;
begin
  Result := minV + Random * (maxV - minV);
end;

function RandBool: Boolean;
begin
  Result := Random(2) = 0;
end;

procedure GenerateFiles();
var
  outDir, fnCsv, fnXls, fullpathCsv, fullpathXls: string;
  f: TextFile;
  ts: string;
  // Data vars
  recAt: TDateTime;
  volt, temp: Double;
  isActive: Boolean;
  statusMsg: string;
begin
  outDir := GetEnvDef('CSV_OUT_DIR', '/data/csv');
  ts := FormatDateTime('yyyymmdd_hhnnss', Now);
  fnCsv := 'telemetry_' + ts + '.csv';
  fnXls := 'telemetry_' + ts + '.xls'; // Using .xls for SpreadsheetML XML
  fullpathCsv := IncludeTrailingPathDelimiter(outDir) + fnCsv;
  fullpathXls := IncludeTrailingPathDelimiter(outDir) + fnXls;

  recAt := Now;
  volt := RandFloat(3.2, 12.6);
  temp := RandFloat(-50.0, 80.0);
  isActive := RandBool;
  if isActive then statusMsg := 'SYSTEM_OK' else statusMsg := 'SYSTEM_OFFLINE';

  AssignFile(f, fullpathCsv);
  Rewrite(f);
  // Header
  Writeln(f, 'recorded_at,voltage,temperature,is_active,status_message');
  // Data row with correct formatting
  Writeln(f, FormatDateTime('yyyy-mm-dd"T"hh:nn:ss"Z"', recAt) + ',' +
             FormatFloat('0.00', volt) + ',' +
             FormatFloat('0.00', temp) + ',' +
             UpperCase(BoolToStr(isActive, 'TRUE', 'FALSE')) + ',' +
             '"' + statusMsg + '"');
  CloseFile(f);

  AssignFile(f, fullpathXls);
  Rewrite(f);
  Writeln(f, '<?xml version="1.0"?>');
  Writeln(f, '<?mso-application progid="Excel.Sheet"?>');
  Writeln(f, '<Workbook xmlns="urn:schemas-microsoft-com:office:spreadsheet"');
  Writeln(f, ' xmlns:o="urn:schemas-microsoft-com:office:office"');
  Writeln(f, ' xmlns:x="urn:schemas-microsoft-com:office:excel"');
  Writeln(f, ' xmlns:ss="urn:schemas-microsoft-com:office:spreadsheet"');
  Writeln(f, ' xmlns:html="http://www.w3.org/TR/REC-html40">');
  Writeln(f, ' <Worksheet ss:Name="Telemetry">');
  Writeln(f, '  <Table>');
  // Header
  Writeln(f, '   <Row>');
  Writeln(f, '    <Cell><Data ss:Type="String">recorded_at</Data></Cell>');
  Writeln(f, '    <Cell><Data ss:Type="String">voltage</Data></Cell>');
  Writeln(f, '    <Cell><Data ss:Type="String">temperature</Data></Cell>');
  Writeln(f, '    <Cell><Data ss:Type="String">is_active</Data></Cell>');
  Writeln(f, '    <Cell><Data ss:Type="String">status_message</Data></Cell>');
  Writeln(f, '   </Row>');
  // Data
  Writeln(f, '   <Row>');
  Writeln(f, '    <Cell><Data ss:Type="String">' + FormatDateTime('yyyy-mm-ddThh:nn:ss', recAt) + '</Data></Cell>');
  Writeln(f, '    <Cell><Data ss:Type="Number">' + FormatFloat('0.00', volt) + '</Data></Cell>');
  Writeln(f, '    <Cell><Data ss:Type="Number">' + FormatFloat('0.00', temp) + '</Data></Cell>');
  Writeln(f, '    <Cell><Data ss:Type="String">' + BoolToStr(isActive, 'TRUE', 'FALSE') + '</Data></Cell>');
  Writeln(f, '    <Cell><Data ss:Type="String">' + statusMsg + '</Data></Cell>');
  Writeln(f, '   </Row>');
  Writeln(f, '  </Table>');
  Writeln(f, ' </Worksheet>');
  Writeln(f, '</Workbook>');
  CloseFile(f);

  ExecuteProcess('/usr/bin/psql', [
    'host=' + GetEnvDef('PGHOST', 'db'),
    'port=' + GetEnvDef('PGPORT', '5432'),
    'user=' + GetEnvDef('PGUSER', 'monouser'),
    'dbname=' + GetEnvDef('PGDATABASE', 'monolith'),
    '-c',
    'INSERT INTO telemetry_legacy(recorded_at, voltage, temp, source_file) VALUES (' +
    '''' + FormatDateTime('yyyy-mm-dd hh:nn:ss', recAt) + ''',' +
    FormatFloat('0.00', volt) + ',' +
    FormatFloat('0.00', temp) + ',' +
    '''' + fnCsv + ''')'
  ]);
end;

var period: Integer;
begin
  Randomize;
  period := StrToIntDef(GetEnvDef('GEN_PERIOD_SEC', '300'), 300);
  while True do
  begin
    try
      GenerateFiles();
    except
      on E: Exception do
        WriteLn('Legacy error: ', E.Message);
    end;
    Sleep(period * 1000);
  end;
end.