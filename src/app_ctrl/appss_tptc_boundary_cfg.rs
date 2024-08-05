#[doc = "Register `APPSS_TPTC_BOUNDARY_CFG` reader"]
pub type R = crate::R<AppssTptcBoundaryCfgSpec>;
#[doc = "Register `APPSS_TPTC_BOUNDARY_CFG` writer"]
pub type W = crate::W<AppssTptcBoundaryCfgSpec>;
#[doc = "Field `tptc_a0_size` reader - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_A0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA0SizeR = crate::FieldReader;
#[doc = "Field `tptc_a0_size` writer - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_A0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA0SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_a1_size` reader - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_A1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA1SizeR = crate::FieldReader;
#[doc = "Field `tptc_a1_size` writer - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_A1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_b0_size` reader - 21:16\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_B0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcB0SizeR = crate::FieldReader;
#[doc = "Field `tptc_b0_size` writer - 21:16\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_B0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcB0SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_b1_size` reader - 29:24\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_B1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcB1SizeR = crate::FieldReader;
#[doc = "Field `tptc_b1_size` writer - 29:24\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_B1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcB1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_A0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    pub fn tptc_a0_size(&self) -> TptcA0SizeR {
        TptcA0SizeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_A1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    pub fn tptc_a1_size(&self) -> TptcA1SizeR {
        TptcA1SizeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_B0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    pub fn tptc_b0_size(&self) -> TptcB0SizeR {
        TptcB0SizeR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_B1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    pub fn tptc_b1_size(&self) -> TptcB1SizeR {
        TptcB1SizeR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_A0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_size(&mut self) -> TptcA0SizeW<AppssTptcBoundaryCfgSpec> {
        TptcA0SizeW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_A1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1_size(&mut self) -> TptcA1SizeW<AppssTptcBoundaryCfgSpec> {
        TptcA1SizeW::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_B0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_size(&mut self) -> TptcB0SizeW<AppssTptcBoundaryCfgSpec> {
        TptcB0SizeW::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC_B1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1_size(&mut self) -> TptcB1SizeW<AppssTptcBoundaryCfgSpec> {
        TptcB1SizeW::new(self, 24)
    }
}
#[doc = "APPSS_TPTC_BOUNDARY_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tptc_boundary_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tptc_boundary_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssTptcBoundaryCfgSpec;
impl crate::RegisterSpec for AppssTptcBoundaryCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_tptc_boundary_cfg::R`](R) reader structure"]
impl crate::Readable for AppssTptcBoundaryCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_tptc_boundary_cfg::W`](W) writer structure"]
impl crate::Writable for AppssTptcBoundaryCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_TPTC_BOUNDARY_CFG to value 0"]
impl crate::Resettable for AppssTptcBoundaryCfgSpec {
    const RESET_VALUE: u32 = 0;
}
