#[doc = "Register `APPSS_MCAN_FE_AND_LIN_INTR_SEL` reader"]
pub type R = crate::R<AppssMcanFeAndLinIntrSelSpec>;
#[doc = "Register `APPSS_MCAN_FE_AND_LIN_INTR_SEL` writer"]
pub type W = crate::W<AppssMcanFeAndLinIntrSelSpec>;
#[doc = "Field `mcan_fe_sel` reader - 2:0\\]
Writing a value 'N' would select Nth filter interrupt combination with SYNC_IN(IO) for triggering timing engine Example: writing 3'd&lt;1-7> selects MCAN_FE_INT&lt;1-7> respectively"]
pub type McanFeSelR = crate::FieldReader;
#[doc = "Field `mcan_fe_sel` writer - 2:0\\]
Writing a value 'N' would select Nth filter interrupt combination with SYNC_IN(IO) for triggering timing engine Example: writing 3'd&lt;1-7> selects MCAN_FE_INT&lt;1-7> respectively"]
pub type McanFeSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `lin_intr_sel` reader - 3:3\\]
Writing a value would select the LIN interrupt in combination with HW_SYNC_IN and CAN filter events for Frame timer 0 : 0th interrupt bit is selected 1 : 1st interrupt bit is selected"]
pub type LinIntrSelR = crate::BitReader;
#[doc = "Field `lin_intr_sel` writer - 3:3\\]
Writing a value would select the LIN interrupt in combination with HW_SYNC_IN and CAN filter events for Frame timer 0 : 0th interrupt bit is selected 1 : 1st interrupt bit is selected"]
pub type LinIntrSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Writing a value 'N' would select Nth filter interrupt combination with SYNC_IN(IO) for triggering timing engine Example: writing 3'd&lt;1-7> selects MCAN_FE_INT&lt;1-7> respectively"]
    #[inline(always)]
    pub fn mcan_fe_sel(&self) -> McanFeSelR {
        McanFeSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing a value would select the LIN interrupt in combination with HW_SYNC_IN and CAN filter events for Frame timer 0 : 0th interrupt bit is selected 1 : 1st interrupt bit is selected"]
    #[inline(always)]
    pub fn lin_intr_sel(&self) -> LinIntrSelR {
        LinIntrSelR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Writing a value 'N' would select Nth filter interrupt combination with SYNC_IN(IO) for triggering timing engine Example: writing 3'd&lt;1-7> selects MCAN_FE_INT&lt;1-7> respectively"]
    #[inline(always)]
    #[must_use]
    pub fn mcan_fe_sel(&mut self) -> McanFeSelW<AppssMcanFeAndLinIntrSelSpec> {
        McanFeSelW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing a value would select the LIN interrupt in combination with HW_SYNC_IN and CAN filter events for Frame timer 0 : 0th interrupt bit is selected 1 : 1st interrupt bit is selected"]
    #[inline(always)]
    #[must_use]
    pub fn lin_intr_sel(&mut self) -> LinIntrSelW<AppssMcanFeAndLinIntrSelSpec> {
        LinIntrSelW::new(self, 3)
    }
}
#[doc = "APPSS_MCAN_FE_AND_LIN_INTR_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mcan_fe_and_lin_intr_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mcan_fe_and_lin_intr_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssMcanFeAndLinIntrSelSpec;
impl crate::RegisterSpec for AppssMcanFeAndLinIntrSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_mcan_fe_and_lin_intr_sel::R`](R) reader structure"]
impl crate::Readable for AppssMcanFeAndLinIntrSelSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_mcan_fe_and_lin_intr_sel::W`](W) writer structure"]
impl crate::Writable for AppssMcanFeAndLinIntrSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_MCAN_FE_AND_LIN_INTR_SEL to value 0"]
impl crate::Resettable for AppssMcanFeAndLinIntrSelSpec {
    const RESET_VALUE: u32 = 0;
}
