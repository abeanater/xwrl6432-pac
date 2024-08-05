#[doc = "Register `DIG_SYNC_SELECT` reader"]
pub type R = crate::R<DigSyncSelectSpec>;
#[doc = "Register `DIG_SYNC_SELECT` writer"]
pub type W = crate::W<DigSyncSelectSpec>;
#[doc = "Field `dig_sync_select` reader - 1:0\\]
Selects dig_sync_in for FRC 2'b00: dig_sync_in 2'b01: mcan_intr 2'b10: lin_intr"]
pub type DigSyncSelectR = crate::FieldReader;
#[doc = "Field `dig_sync_select` writer - 1:0\\]
Selects dig_sync_in for FRC 2'b00: dig_sync_in 2'b01: mcan_intr 2'b10: lin_intr"]
pub type DigSyncSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selects dig_sync_in for FRC 2'b00: dig_sync_in 2'b01: mcan_intr 2'b10: lin_intr"]
    #[inline(always)]
    pub fn dig_sync_select(&self) -> DigSyncSelectR {
        DigSyncSelectR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selects dig_sync_in for FRC 2'b00: dig_sync_in 2'b01: mcan_intr 2'b10: lin_intr"]
    #[inline(always)]
    #[must_use]
    pub fn dig_sync_select(&mut self) -> DigSyncSelectW<DigSyncSelectSpec> {
        DigSyncSelectW::new(self, 0)
    }
}
#[doc = "DIG_SYNC_SELECT\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_sync_select::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_sync_select::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DigSyncSelectSpec;
impl crate::RegisterSpec for DigSyncSelectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dig_sync_select::R`](R) reader structure"]
impl crate::Readable for DigSyncSelectSpec {}
#[doc = "`write(|w| ..)` method takes [`dig_sync_select::W`](W) writer structure"]
impl crate::Writable for DigSyncSelectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIG_SYNC_SELECT to value 0"]
impl crate::Resettable for DigSyncSelectSpec {
    const RESET_VALUE: u32 = 0;
}
