#[doc = "Register `DEBUG_STATUS_AON_15` reader"]
pub type R = crate::R<DebugStatusAon15Spec>;
#[doc = "Register `DEBUG_STATUS_AON_15` writer"]
pub type W = crate::W<DebugStatusAon15Spec>;
#[doc = "Field `DIS_JTAG` reader - 0:0\\]
status reg for DIS_JTAG"]
pub type DisJtagR = crate::BitReader;
#[doc = "Field `DIS_JTAG` writer - 0:0\\]
status reg for DIS_JTAG"]
pub type DisJtagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DIS` reader - 1:1\\]
status reg for JTAG_DIS"]
pub type JtagDisR = crate::BitReader;
#[doc = "Field `JTAG_DIS` writer - 1:1\\]
status reg for JTAG_DIS"]
pub type JtagDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS232_DIS` reader - 2:2\\]
status reg for RS232_DIS"]
pub type Rs232DisR = crate::BitReader;
#[doc = "Field `RS232_DIS` writer - 2:2\\]
status reg for RS232_DIS"]
pub type Rs232DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_DIS` reader - 3:3\\]
status reg for TEST_DIS"]
pub type TestDisR = crate::BitReader;
#[doc = "Field `TEST_DIS` writer - 3:3\\]
status reg for TEST_DIS"]
pub type TestDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVDS_DIS` reader - 4:4\\]
status reg for LVDS_DIS"]
pub type LvdsDisR = crate::BitReader;
#[doc = "Field `LVDS_DIS` writer - 4:4\\]
status reg for LVDS_DIS"]
pub type LvdsDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
status reg for DIS_JTAG"]
    #[inline(always)]
    pub fn dis_jtag(&self) -> DisJtagR {
        DisJtagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for JTAG_DIS"]
    #[inline(always)]
    pub fn jtag_dis(&self) -> JtagDisR {
        JtagDisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for RS232_DIS"]
    #[inline(always)]
    pub fn rs232_dis(&self) -> Rs232DisR {
        Rs232DisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for TEST_DIS"]
    #[inline(always)]
    pub fn test_dis(&self) -> TestDisR {
        TestDisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for LVDS_DIS"]
    #[inline(always)]
    pub fn lvds_dis(&self) -> LvdsDisR {
        LvdsDisR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
status reg for DIS_JTAG"]
    #[inline(always)]
    #[must_use]
    pub fn dis_jtag(&mut self) -> DisJtagW<DebugStatusAon15Spec> {
        DisJtagW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for JTAG_DIS"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dis(&mut self) -> JtagDisW<DebugStatusAon15Spec> {
        JtagDisW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for RS232_DIS"]
    #[inline(always)]
    #[must_use]
    pub fn rs232_dis(&mut self) -> Rs232DisW<DebugStatusAon15Spec> {
        Rs232DisW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for TEST_DIS"]
    #[inline(always)]
    #[must_use]
    pub fn test_dis(&mut self) -> TestDisW<DebugStatusAon15Spec> {
        TestDisW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for LVDS_DIS"]
    #[inline(always)]
    #[must_use]
    pub fn lvds_dis(&mut self) -> LvdsDisW<DebugStatusAon15Spec> {
        LvdsDisW::new(self, 4)
    }
}
#[doc = "DEBUG_STATUS_AON_15\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon15Spec;
impl crate::RegisterSpec for DebugStatusAon15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_15::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon15Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_15::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_15 to value 0"]
impl crate::Resettable for DebugStatusAon15Spec {
    const RESET_VALUE: u32 = 0;
}
