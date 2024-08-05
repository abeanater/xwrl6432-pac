#[doc = "Register `ERROR_STATUS3` reader"]
pub type R = crate::R<ErrorStatus3Spec>;
#[doc = "Register `ERROR_STATUS3` writer"]
pub type W = crate::W<ErrorStatus3Spec>;
#[doc = "Field `WB_PEND` reader - 0:0\\]
delayed write back pending Status - (RO )"]
pub type WbPendR = crate::BitReader;
#[doc = "Field `WB_PEND` writer - 0:0\\]
delayed write back pending Status - (RO )"]
pub type WbPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVBUS_TIMEOUT_ERR` reader - 1:1\\]
Level svbus timeout error Error Status - (RW )"]
pub type SvbusTimeoutErrR = crate::BitReader;
#[doc = "Field `SVBUS_TIMEOUT_ERR` writer - 1:1\\]
Level svbus timeout error Error Status - (RW )"]
pub type SvbusTimeoutErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES6` reader - 8:2\\]
RESERVE FIELD"]
pub type Res6R = crate::FieldReader;
#[doc = "Field `RES6` writer - 8:2\\]
RESERVE FIELD"]
pub type Res6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CLR_SVBUS_TIMEOUT_ERR` reader - 9:9\\]
Clear svbus timeout Error Status - (RW )"]
pub type ClrSvbusTimeoutErrR = crate::BitReader;
#[doc = "Field `CLR_SVBUS_TIMEOUT_ERR` writer - 9:9\\]
Clear svbus timeout Error Status - (RW )"]
pub type ClrSvbusTimeoutErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES5` reader - 31:10\\]
RESERVE FIELD"]
pub type Res5R = crate::FieldReader<u32>;
#[doc = "Field `RES5` writer - 31:10\\]
RESERVE FIELD"]
pub type Res5W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
delayed write back pending Status - (RO )"]
    #[inline(always)]
    pub fn wb_pend(&self) -> WbPendR {
        WbPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Level svbus timeout error Error Status - (RW )"]
    #[inline(always)]
    pub fn svbus_timeout_err(&self) -> SvbusTimeoutErrR {
        SvbusTimeoutErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:8 - 8:2\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res6(&self) -> Res6R {
        Res6R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear svbus timeout Error Status - (RW )"]
    #[inline(always)]
    pub fn clr_svbus_timeout_err(&self) -> ClrSvbusTimeoutErrR {
        ClrSvbusTimeoutErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res5(&self) -> Res5R {
        Res5R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
delayed write back pending Status - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn wb_pend(&mut self) -> WbPendW<ErrorStatus3Spec> {
        WbPendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Level svbus timeout error Error Status - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn svbus_timeout_err(&mut self) -> SvbusTimeoutErrW<ErrorStatus3Spec> {
        SvbusTimeoutErrW::new(self, 1)
    }
    #[doc = "Bits 2:8 - 8:2\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res6(&mut self) -> Res6W<ErrorStatus3Spec> {
        Res6W::new(self, 2)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear svbus timeout Error Status - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn clr_svbus_timeout_err(&mut self) -> ClrSvbusTimeoutErrW<ErrorStatus3Spec> {
        ClrSvbusTimeoutErrW::new(self, 9)
    }
    #[doc = "Bits 10:31 - 31:10\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res5(&mut self) -> Res5W<ErrorStatus3Spec> {
        Res5W::new(self, 10)
    }
}
#[doc = "ECC Error Status3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_status3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_status3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorStatus3Spec;
impl crate::RegisterSpec for ErrorStatus3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_status3::R`](R) reader structure"]
impl crate::Readable for ErrorStatus3Spec {}
#[doc = "`write(|w| ..)` method takes [`error_status3::W`](W) writer structure"]
impl crate::Writable for ErrorStatus3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERROR_STATUS3 to value 0"]
impl crate::Resettable for ErrorStatus3Spec {
    const RESET_VALUE: u32 = 0;
}
