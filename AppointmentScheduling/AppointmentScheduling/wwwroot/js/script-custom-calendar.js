﻿var routeURL = location.protocol + '//' + location.host;

$(document).ready(function() {
    InitializeCalendar();
});

function InitializeCalendar() {
    try {

        var calendarEl = document.getElementById('calendar');
        if (calendarEl != null) {
            var calendar = new FullCalendar.Calendar(calendarEl, {
              initialView: 'dayGridMonth',
              headerToolbar: {
                  left: 'prev,next,today',
                  center: 'title', 
                  right: 'dayGridMonth,timeGridWeek,timeGridDay'
              },
              selectable: true,
              editable: false,
              select: function (event) {
                  onShowModal(event, null);
              }
            });
    
            calendar.render();
        }
    }
    catch (e) {
        alert(e);
    }
}

function onShowModal(obj, isEventDetail) {
    $("#appointmentInput").modal("show");
}

function onCloseModal() {
    $("#appointmentInput").modal("hide");
}

function onSubmitForm() {
    var requestData = {
        Id: parserInt($("#id").val()),
        Title: $("#title").val(),
        Description: $("#description").val(),
        StartDate: $("#appointmentDate").val(),
        Duration: $("#duration").val(),
        DoctorId: $("#doctorId").val(),
        PatientId: $("#patientId").val()
    };
}