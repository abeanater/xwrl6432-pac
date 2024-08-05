#[doc = "Register `AETCMD` reader"]
pub type R = crate::R<AetcmdSpec>;
#[doc = "Register `AETCMD` writer"]
pub type W = crate::W<AetcmdSpec>;
#[doc = "Field `CLR` reader - 0:0\\]
AET Clear command: CPU write of '1' to the CLR bit causes the tpcc_aet output signal and AETSTAT.STAT register to be cleared. CPU write of '0' has no effect.."]
pub type ClrR = crate::BitReader;
#[doc = "Field `CLR` writer - 0:0\\]
AET Clear command: CPU write of '1' to the CLR bit causes the tpcc_aet output signal and AETSTAT.STAT register to be cleared. CPU write of '0' has no effect.."]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES68` reader - 31:1\\]
RESERVE FIELD"]
pub type Res68R = crate::FieldReader<u32>;
#[doc = "Field `RES68` writer - 31:1\\]
RESERVE FIELD"]
pub type Res68W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AET Clear command: CPU write of '1' to the CLR bit causes the tpcc_aet output signal and AETSTAT.STAT register to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res68(&self) -> Res68R {
        Res68R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
AET Clear command: CPU write of '1' to the CLR bit causes the tpcc_aet output signal and AETSTAT.STAT register to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> ClrW<AetcmdSpec> {
        ClrW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res68(&mut self) -> Res68W<AetcmdSpec> {
        Res68W::new(self, 1)
    }
}
#[doc = "AET Command\n\nYou can [`read`](crate::Reg::read) this register and get [`aetcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aetcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AetcmdSpec;
impl crate::RegisterSpec for AetcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aetcmd::R`](R) reader structure"]
impl crate::Readable for AetcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`aetcmd::W`](W) writer structure"]
impl crate::Writable for AetcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AETCMD to value 0"]
impl crate::Resettable for AetcmdSpec {
    const RESET_VALUE: u32 = 0;
}
