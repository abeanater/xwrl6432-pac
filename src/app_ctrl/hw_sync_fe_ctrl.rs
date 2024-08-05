#[doc = "Register `HW_Sync_FE_CTRL` reader"]
pub type R = crate::R<HwSyncFeCtrlSpec>;
#[doc = "Register `HW_Sync_FE_CTRL` writer"]
pub type W = crate::W<HwSyncFeCtrlSpec>;
#[doc = "Field `fe1_sel` reader - 0:0\\]
RESERVED"]
pub type Fe1SelR = crate::BitReader;
#[doc = "Field `fe1_sel` writer - 0:0\\]
RESERVED"]
pub type Fe1SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fe2_sel` reader - 8:8\\]
RESERVED"]
pub type Fe2SelR = crate::BitReader;
#[doc = "Field `fe2_sel` writer - 8:8\\]
RESERVED"]
pub type Fe2SelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RESERVED"]
    #[inline(always)]
    pub fn fe1_sel(&self) -> Fe1SelR {
        Fe1SelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED"]
    #[inline(always)]
    pub fn fe2_sel(&self) -> Fe2SelR {
        Fe2SelR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn fe1_sel(&mut self) -> Fe1SelW<HwSyncFeCtrlSpec> {
        Fe1SelW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn fe2_sel(&mut self) -> Fe2SelW<HwSyncFeCtrlSpec> {
        Fe2SelW::new(self, 8)
    }
}
#[doc = "HW_Sync_FE_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_sync_fe_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_sync_fe_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwSyncFeCtrlSpec;
impl crate::RegisterSpec for HwSyncFeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_sync_fe_ctrl::R`](R) reader structure"]
impl crate::Readable for HwSyncFeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hw_sync_fe_ctrl::W`](W) writer structure"]
impl crate::Writable for HwSyncFeCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_Sync_FE_CTRL to value 0"]
impl crate::Resettable for HwSyncFeCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
