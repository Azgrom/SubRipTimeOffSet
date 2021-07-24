﻿using AppointmentScheduling.Services;
using Microsoft.AspNetCore.Mvc;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AppointmentScheduling.Controllers
{
    public class AppointmentController : Controller
    {
        private readonly IAppointmentService _appointment_service;

        public AppointmentController(IAppointmentService appointment_service)
        {
            _appointment_service = appointment_service;
        }

        public IActionResult Index()
        {
            _appointment_service.GetDoctorList();
            return View();
        }
    }
}
