using System;
using System.Security.Claims;
using AppointmentScheduling.Models.ViewModels;
using AppointmentScheduling.Services;
using AppointmentScheduling.Utility;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;

namespace AppointmentScheduling.Controllers.API
{
    /// <summary>
    /// 
    /// </summary>
    [Route(template: "API/Appointment")]
    [ApiController]
    public class AppointmentApiController : Controller
    {
        private readonly IAppointmentService _appointmentService;
        private readonly IHttpContextAccessor _httpContextAccessor;
        private readonly string _loginUserId;
        private readonly string _role;

        public AppointmentApiController(IAppointmentService appointmentService, 
                                        IHttpContextAccessor httpContextAccessor)
        {
            _appointmentService = appointmentService;
            _httpContextAccessor = httpContextAccessor;
            _loginUserId = _httpContextAccessor.HttpContext?.User.FindFirstValue(claimType: ClaimTypes.NameIdentifier);
            _role = _httpContextAccessor.HttpContext?.User.FindFirstValue(claimType: ClaimTypes.Role);
        }

        [HttpPost]
        [Route(template: "SaveCalendarData")]
        public IActionResult SaveCalendarData(AppointmentViewModel data)
        {
            var commonResponse = new CommonResponse<int>();
            try
            {
                commonResponse.Status = _appointmentService.AddUpdate(data).Result;

                commonResponse.Message = commonResponse.Status switch
                {
                    1 => Helper.AppointmentUpdated,
                    2 => Helper.AppointmentAdded,
                    _ => Helper.SomethingWentWrong,
                };
            }
            catch (Exception e)
            {
                commonResponse.Message = e.Message;
                commonResponse.Status = Helper.FailureCode;
            }

            return Ok(commonResponse);
        }
    }
}
