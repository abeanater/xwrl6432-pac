#[doc = "Register `MCRC_BUS_SEL` reader"]
pub type R = crate::R<McrcBusSelSpec>;
#[doc = "Register `MCRC_BUS_SEL` writer"]
pub type W = crate::W<McrcBusSelSpec>;
#[doc = "Field `ITCMEn` reader - 0:0\\]
ITCMEn. Enable/disables the tracing of instruction TCM 0: Tracing of ITCM bus has been disabled 1: Tracing of ITCM bus has been enabled"]
pub type ItcmenR = crate::BitReader;
#[doc = "Field `ITCMEn` writer - 0:0\\]
ITCMEn. Enable/disables the tracing of instruction TCM 0: Tracing of ITCM bus has been disabled 1: Tracing of ITCM bus has been enabled"]
pub type ItcmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCMEn` reader - 1:1\\]
DTCMEn. Enable/disables the tracing of data TCM 0: Tracing of DTCM_ODD and DTCM_EVEN buses have been disabled 1: Tracing of DTCM_ODD and DTCM_EVEN buses have been enabled"]
pub type DtcmenR = crate::BitReader;
#[doc = "Field `DTCMEn` writer - 1:1\\]
DTCMEn. Enable/disables the tracing of data TCM 0: Tracing of DTCM_ODD and DTCM_EVEN buses have been disabled 1: Tracing of DTCM_ODD and DTCM_EVEN buses have been enabled"]
pub type DtcmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEn` reader - 2:2\\]
MEn. Enable/disables the tracing of VBUSM 0: Tracing of VBUSM master bus has been disabled 1: Tracing of VBUSM master bus has been enabled"]
pub type MenR = crate::BitReader;
#[doc = "Field `MEn` writer - 2:2\\]
MEn. Enable/disables the tracing of VBUSM 0: Tracing of VBUSM master bus has been disabled 1: Tracing of VBUSM master bus has been enabled"]
pub type MenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU67` reader - 31:3\\]
Reserved"]
pub type Nu67R = crate::FieldReader<u32>;
#[doc = "Field `NU67` writer - 31:3\\]
Reserved"]
pub type Nu67W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ITCMEn. Enable/disables the tracing of instruction TCM 0: Tracing of ITCM bus has been disabled 1: Tracing of ITCM bus has been enabled"]
    #[inline(always)]
    pub fn itcmen(&self) -> ItcmenR {
        ItcmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DTCMEn. Enable/disables the tracing of data TCM 0: Tracing of DTCM_ODD and DTCM_EVEN buses have been disabled 1: Tracing of DTCM_ODD and DTCM_EVEN buses have been enabled"]
    #[inline(always)]
    pub fn dtcmen(&self) -> DtcmenR {
        DtcmenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
MEn. Enable/disables the tracing of VBUSM 0: Tracing of VBUSM master bus has been disabled 1: Tracing of VBUSM master bus has been enabled"]
    #[inline(always)]
    pub fn men(&self) -> MenR {
        MenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved"]
    #[inline(always)]
    pub fn nu67(&self) -> Nu67R {
        Nu67R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ITCMEn. Enable/disables the tracing of instruction TCM 0: Tracing of ITCM bus has been disabled 1: Tracing of ITCM bus has been enabled"]
    #[inline(always)]
    #[must_use]
    pub fn itcmen(&mut self) -> ItcmenW<McrcBusSelSpec> {
        ItcmenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DTCMEn. Enable/disables the tracing of data TCM 0: Tracing of DTCM_ODD and DTCM_EVEN buses have been disabled 1: Tracing of DTCM_ODD and DTCM_EVEN buses have been enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dtcmen(&mut self) -> DtcmenW<McrcBusSelSpec> {
        DtcmenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
MEn. Enable/disables the tracing of VBUSM 0: Tracing of VBUSM master bus has been disabled 1: Tracing of VBUSM master bus has been enabled"]
    #[inline(always)]
    #[must_use]
    pub fn men(&mut self) -> MenW<McrcBusSelSpec> {
        MenW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu67(&mut self) -> Nu67W<McrcBusSelSpec> {
        Nu67W::new(self, 3)
    }
}
#[doc = "Disables either or all tracing of data buses\n\nYou can [`read`](crate::Reg::read) this register and get [`mcrc_bus_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcrc_bus_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrcBusSelSpec;
impl crate::RegisterSpec for McrcBusSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc_bus_sel::R`](R) reader structure"]
impl crate::Readable for McrcBusSelSpec {}
#[doc = "`write(|w| ..)` method takes [`mcrc_bus_sel::W`](W) writer structure"]
impl crate::Writable for McrcBusSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC_BUS_SEL to value 0"]
impl crate::Resettable for McrcBusSelSpec {
    const RESET_VALUE: u32 = 0;
}
